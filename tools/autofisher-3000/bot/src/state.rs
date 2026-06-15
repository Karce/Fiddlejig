//! The fishing loop as a pure, typed state machine.
//!
//! [`step`] takes the current [`FishingState`] plus a [`StepCtx`] snapshot and
//! returns the next state and the [`Action`]s the shell should perform. It does no
//! I/O, reads no clock, and never sleeps — timing is encoded as deadlines carried
//! in the state — so every transition is deterministically unit-testable.

use crate::config::Config;
use std::time::Instant;

/// A point in frame coordinates. Thanks to the RemoteDesktop portal these are also
/// the pointer-injection coordinates, so there is no capture→screen calibration.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A side effect the state machine asks the shell to perform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    /// Press and release a key, given as a Linux evdev keycode.
    PressKey(u16),
    /// Move the pointer to a frame coordinate.
    MoveTo(Point),
    /// Right-click at the current pointer position (reels in the bobber).
    RightClick,
}

/// A bobber being confirmed before the bot reels: it must persist near `pos` for
/// `Config::stability`, tolerating brief detection flicker (`Config::flicker_grace`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pos: Point,
    /// When this detection streak began.
    stable_since: Instant,
    /// Most recent sighting (drives the flicker tolerance).
    last_seen: Instant,
}

/// The fishing loop. Associated data makes invalid states unrepresentable — e.g. a
/// recast deadline exists only while `Searching`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FishingState {
    /// Startup grace period so the user can focus the game.
    Initializing { until: Instant },
    /// Waiting for the lure to finish applying before casting.
    ApplyingLure { until: Instant },
    /// Ready to (re)cast on the next step.
    Casting,
    /// Line just cast — ignore the water until `until` so the cast animation and
    /// the idle bobber don't trigger a premature reel or a false positive.
    Settling { until: Instant },
    /// Line is out; confirm a stable bobber then reel, or recast once `recast_at`
    /// passes. `candidate` is the bobber currently being confirmed.
    Searching {
        recast_at: Instant,
        candidate: Option<Candidate>,
    },
    /// A fish was just reeled in — pause briefly so the catch/loot resolves before
    /// recasting, so a lure re-apply keypress isn't swallowed by the loot moment.
    Looting { until: Instant },
}

impl FishingState {
    /// Initial state: begin the startup grace period.
    pub fn new(now: Instant, cfg: &Config) -> Self {
        FishingState::Initializing {
            until: now + cfg.initializing(),
        }
    }
}

/// Everything a single [`step`] needs — an immutable snapshot of the world.
pub struct StepCtx<'a> {
    pub now: Instant,
    /// Detected bobber centers, in frame coordinates.
    pub targets: &'a [Point],
    /// Frame center; the nearest bobber to it is preferred.
    pub center: Point,
    /// `Some(true/false)` when lure detection ran, `None` when it is disabled.
    pub lure_present: Option<bool>,
    pub cfg: &'a Config,
}

/// Advance the state machine one tick.
pub fn step(state: FishingState, ctx: &StepCtx) -> (FishingState, Vec<Action>) {
    match state {
        FishingState::Initializing { until } => {
            if ctx.now >= until {
                begin_cast(ctx)
            } else {
                (state, Vec::new())
            }
        }
        FishingState::ApplyingLure { until } => {
            if ctx.now >= until {
                cast_now(ctx)
            } else {
                (state, Vec::new())
            }
        }
        FishingState::Casting => begin_cast(ctx),
        FishingState::Settling { until } => {
            if ctx.now >= until {
                // line has settled; start the bite window now
                (
                    FishingState::Searching {
                        recast_at: ctx.now + ctx.cfg.fishing_duration(),
                        candidate: None,
                    },
                    Vec::new(),
                )
            } else {
                (state, Vec::new())
            }
        }
        FishingState::Searching {
            recast_at,
            candidate,
        } => {
            let nearest = nearest(ctx.targets, ctx.center);
            let candidate = update_candidate(candidate, nearest, ctx.now, ctx.cfg);
            match candidate {
                // confirmed stable long enough → reel, then pause to loot before recasting
                Some(c) if ctx.now >= c.stable_since + ctx.cfg.stability() => (
                    FishingState::Looting {
                        until: ctx.now + ctx.cfg.post_catch(),
                    },
                    vec![Action::MoveTo(c.pos), Action::RightClick],
                ),
                // still confirming a sighting
                Some(c) => (
                    FishingState::Searching {
                        recast_at,
                        candidate: Some(c),
                    },
                    Vec::new(),
                ),
                // nothing on the water; recast once the bite window expires
                None if ctx.now >= recast_at => (FishingState::Casting, Vec::new()),
                None => (
                    FishingState::Searching {
                        recast_at,
                        candidate: None,
                    },
                    Vec::new(),
                ),
            }
        }
        FishingState::Looting { until } => {
            if ctx.now >= until {
                begin_cast(ctx)
            } else {
                (state, Vec::new())
            }
        }
    }
}

/// Apply a lure first if detection says it's missing, otherwise cast straight away.
fn begin_cast(ctx: &StepCtx) -> (FishingState, Vec<Action>) {
    // only (re)apply when detection positively reports the lure is absent
    if ctx.lure_present == Some(false) {
        if let Some(code) = keycode(ctx.cfg.lure_key) {
            return (
                FishingState::ApplyingLure {
                    until: ctx.now + ctx.cfg.lure_cast(),
                },
                vec![Action::PressKey(code)],
            );
        }
    }
    cast_now(ctx)
}

/// Emit the cast keypress(es), park the cursor, and start searching.
fn cast_now(ctx: &StepCtx) -> (FishingState, Vec<Action>) {
    let mut actions: Vec<Action> = ctx
        .cfg
        .cast_keys
        .iter()
        .filter_map(|&c| keycode(c))
        .map(Action::PressKey)
        .collect();
    actions.push(Action::MoveTo(ctx.cfg.mouse_park()));
    // settle before watching the water (the bite window opens after this)
    (
        FishingState::Settling {
            until: ctx.now + ctx.cfg.settle(),
        },
        actions,
    )
}

/// Nearest target to `center` by Euclidean distance.
fn nearest(targets: &[Point], center: Point) -> Option<Point> {
    targets.iter().copied().min_by(|a, b| {
        dist2(*a, center)
            .partial_cmp(&dist2(*b, center))
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn dist2(a: Point, b: Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

/// Advance the confirmation streak: extend it on a nearby sighting, start a fresh
/// one on a new sighting, and keep it alive across brief detection drop-outs (real
/// bobber detection flickers). Returns `None` once the streak lapses.
fn update_candidate(
    candidate: Option<Candidate>,
    nearest: Option<Point>,
    now: Instant,
    cfg: &Config,
) -> Option<Candidate> {
    let radius2 = cfg.stability_radius * cfg.stability_radius;
    match (candidate, nearest) {
        // a sighting near the current streak extends it (keep the start time)
        (Some(c), Some(p)) if dist2(c.pos, p) <= radius2 => Some(Candidate {
            pos: p,
            stable_since: c.stable_since,
            last_seen: now,
        }),
        // any other sighting starts a fresh streak
        (_, Some(p)) => Some(Candidate {
            pos: p,
            stable_since: now,
            last_seen: now,
        }),
        // no sighting this tick: hold the streak only within the flicker grace
        (Some(c), None) if now <= c.last_seen + cfg.flicker_grace() => Some(c),
        (_, None) => None,
    }
}

/// Map a digit character to its Linux evdev keycode (KEY_1=2 … KEY_9=10, KEY_0=11).
pub fn keycode(c: char) -> Option<u16> {
    match c {
        '1'..='9' => Some(c as u16 - '1' as u16 + 2),
        '0' => Some(11),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    fn cfg() -> Config {
        Config::default() // skip_lure=true, cast_keys=['1'], lure_key='3', 10s/30s/10s
    }

    fn ctx<'a>(
        now: Instant,
        targets: &'a [Point],
        lure: Option<bool>,
        c: &'a Config,
    ) -> StepCtx<'a> {
        StepCtx {
            now,
            targets,
            center: ORIGIN,
            lure_present: lure,
            cfg: c,
        }
    }

    #[test]
    fn keycodes_map_digits() {
        assert_eq!(keycode('1'), Some(2));
        assert_eq!(keycode('3'), Some(4));
        assert_eq!(keycode('9'), Some(10));
        assert_eq!(keycode('0'), Some(11));
        assert_eq!(keycode('x'), None);
    }

    #[test]
    fn initializing_holds_then_casts() {
        let c = cfg();
        let t0 = Instant::now();
        let s = FishingState::Initializing {
            until: t0 + Duration::from_secs(10),
        };

        // before the deadline: no change, no actions
        let (s1, a1) = step(s, &ctx(t0, &[], None, &c));
        assert_eq!(s1, s);
        assert!(a1.is_empty());

        // after the deadline: casts (skip_lure → straight to cast) and starts searching
        let after = t0 + Duration::from_secs(11);
        let (s2, a2) = step(s, &ctx(after, &[], None, &c));
        assert!(matches!(s2, FishingState::Settling { .. }));
        assert_eq!(
            a2,
            vec![Action::PressKey(2), Action::MoveTo(c.mouse_park())]
        );
    }

    #[test]
    fn cast_uses_all_configured_keys() {
        let mut c = cfg();
        c.cast_keys = vec!['1', '2'];
        let t0 = Instant::now();
        let (_s, a) = step(FishingState::Casting, &ctx(t0, &[], None, &c));
        assert_eq!(
            a,
            vec![
                Action::PressKey(2),
                Action::PressKey(3),
                Action::MoveTo(c.mouse_park())
            ]
        );
    }

    #[test]
    fn lure_missing_applies_lure_then_casts() {
        let mut c = cfg();
        c.skip_lure = false;
        let t0 = Instant::now();

        // lure reported absent → press lure key, wait
        let (s1, a1) = step(FishingState::Casting, &ctx(t0, &[], Some(false), &c));
        assert!(matches!(s1, FishingState::ApplyingLure { .. }));
        assert_eq!(a1, vec![Action::PressKey(4)]); // KEY_3

        // not yet done applying → hold
        if let FishingState::ApplyingLure { until } = s1 {
            let (s2, a2) = step(
                s1,
                &ctx(until - Duration::from_secs(1), &[], Some(false), &c),
            );
            assert_eq!(s2, s1);
            assert!(a2.is_empty());

            // done applying → cast
            let (s3, a3) = step(s1, &ctx(until, &[], Some(false), &c));
            assert!(matches!(s3, FishingState::Settling { .. }));
            assert_eq!(
                a3,
                vec![Action::PressKey(2), Action::MoveTo(c.mouse_park())]
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn lure_present_skips_straight_to_cast() {
        let mut c = cfg();
        c.skip_lure = false;
        let t0 = Instant::now();
        let (s, a) = step(FishingState::Casting, &ctx(t0, &[], Some(true), &c));
        assert!(matches!(s, FishingState::Settling { .. }));
        assert_eq!(a, vec![Action::PressKey(2), Action::MoveTo(c.mouse_park())]);
    }

    #[test]
    fn settling_holds_then_searches_ignoring_targets() {
        let c = cfg();
        let t0 = Instant::now();
        let s = FishingState::Settling {
            until: t0 + Duration::from_secs(3),
        };
        // a bobber on screen must NOT be reeled while the line is still settling
        let targets = [Point { x: 5.0, y: 5.0 }];
        let (s1, a1) = step(s, &ctx(t0, &targets, None, &c));
        assert_eq!(s1, s);
        assert!(a1.is_empty(), "must not reel during the settle window");
        // after settling, the bite window opens
        let (s2, a2) = step(s, &ctx(t0 + Duration::from_secs(3), &targets, None, &c));
        assert!(matches!(s2, FishingState::Searching { .. }));
        assert!(a2.is_empty());
    }

    fn searching(recast_in_secs: u64, t0: Instant) -> FishingState {
        FishingState::Searching {
            recast_at: t0 + Duration::from_secs(recast_in_secs),
            candidate: None,
        }
    }

    #[test]
    fn searching_reels_nearest_only_after_it_is_stable() {
        let c = cfg();
        let t0 = Instant::now();
        let s = searching(30, t0);
        // two bobbers; the closer one (3,4 dist 5) wins over (10,0) (center is ORIGIN)
        let targets = [Point { x: 10.0, y: 0.0 }, Point { x: 3.0, y: 4.0 }];

        // first sighting: start confirming — do NOT reel yet
        let (s1, a1) = step(s, &ctx(t0, &targets, None, &c));
        assert!(matches!(
            s1,
            FishingState::Searching {
                candidate: Some(_),
                ..
            }
        ));
        assert!(a1.is_empty());

        // still there after the stability window → reel the nearest
        let later = t0 + c.stability() + Duration::from_millis(1);
        let (s2, a2) = step(s1, &ctx(later, &targets, None, &c));
        assert!(matches!(s2, FishingState::Looting { .. }));
        assert_eq!(
            a2,
            vec![Action::MoveTo(Point { x: 3.0, y: 4.0 }), Action::RightClick]
        );
    }

    #[test]
    fn searching_survives_brief_flicker() {
        let c = cfg();
        let t0 = Instant::now();
        let targets = [Point { x: 3.0, y: 4.0 }];

        let (s1, _) = step(searching(30, t0), &ctx(t0, &targets, None, &c));
        // a drop-out within the flicker grace must NOT reset the streak
        let (s2, _) = step(s1, &ctx(t0 + Duration::from_millis(100), &[], None, &c));
        assert!(matches!(
            s2,
            FishingState::Searching {
                candidate: Some(_),
                ..
            }
        ));
        // re-sighted past the stability window → reel
        let later = t0 + c.stability() + Duration::from_millis(1);
        let (s3, a3) = step(s2, &ctx(later, &targets, None, &c));
        assert!(matches!(s3, FishingState::Looting { .. }));
        assert_eq!(
            a3,
            vec![Action::MoveTo(Point { x: 3.0, y: 4.0 }), Action::RightClick]
        );
    }

    #[test]
    fn looting_pauses_then_recasts() {
        let c = cfg();
        let t0 = Instant::now();
        let s = FishingState::Looting {
            until: t0 + c.post_catch(),
        };
        // during the loot pause: hold, no actions
        let (s1, a1) = step(s, &ctx(t0, &[], None, &c));
        assert_eq!(s1, s);
        assert!(a1.is_empty());
        // after the pause: (re)cast (skip_lure path → straight to a cast)
        let after = t0 + c.post_catch() + Duration::from_millis(1);
        let (s2, a2) = step(s, &ctx(after, &[], None, &c));
        assert!(matches!(s2, FishingState::Settling { .. }));
        assert_eq!(
            a2,
            vec![Action::PressKey(2), Action::MoveTo(c.mouse_park())]
        );
    }

    #[test]
    fn searching_drops_transient_false_positive() {
        let c = cfg();
        let t0 = Instant::now();
        let targets = [Point { x: 3.0, y: 4.0 }];

        let (s1, _) = step(searching(30, t0), &ctx(t0, &targets, None, &c)); // one-frame blip
                                                                             // gone for longer than the flicker grace → streak dropped, never reels
        let after_grace = t0 + c.flicker_grace() + Duration::from_millis(1);
        let (s2, a2) = step(s1, &ctx(after_grace, &[], None, &c));
        assert!(matches!(
            s2,
            FishingState::Searching {
                candidate: None,
                ..
            }
        ));
        assert!(a2.is_empty());
    }

    #[test]
    fn searching_recasts_on_timeout_only() {
        let c = cfg();
        let t0 = Instant::now();
        let s = searching(30, t0);

        // no targets, before deadline → keep waiting
        let (s1, a1) = step(s, &ctx(t0, &[], None, &c));
        assert_eq!(s1, s);
        assert!(a1.is_empty());

        // no targets, deadline passed → recast
        let (s2, a2) = step(s, &ctx(t0 + Duration::from_secs(31), &[], None, &c));
        assert_eq!(s2, FishingState::Casting);
        assert!(a2.is_empty());
    }
}
