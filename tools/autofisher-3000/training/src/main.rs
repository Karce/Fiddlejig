use std::time::{Duration, Instant};
use opencv::core::Size;
//use dbus::blocking::Connection;
use portal_screencast::ScreenCast;
use portal_screencast::ActiveScreenCast;
use portal_screencast::ScreenCastStream;
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::highgui;
use simple_moving_average::SMA;
use simple_moving_average::SingleSumSMA;
use opencv::imgcodecs::imwrite_def;
use opencv::imgproc::resize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    //let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    //let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    //let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    // Let's print all the names to stdout.
    //for name in names { println!("{}", name); }

    let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let screen_cast = start_screencast()?;
    // The screen cast must be kept in scope or else it gets closed.
    let fd = screen_cast.pipewire_fd();
    let streams: Vec<&ScreenCastStream> = screen_cast.streams().collect();
    let node_id: u32 = streams.first().unwrap().pipewire_node();
    let gst_string = format!("pipewiresrc fd={} path={} ! videoconvert ! appsink", fd, node_id);
    println!("gst_string: {}", gst_string);
    
    let mut video = video_capture(gst_string)?;
    let opened = VideoCapture::is_opened(&video)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
    let mut loop_time = Instant::now();
    let mut ma = SingleSumSMA::<_, f32, 15>::new(); // Sample window size = 2

    loop {
        let mut frame = Mat::default();
        video.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // Resize down to 1920x1080.
            // Then display it.
            let mut disp_frm = Mat::default();
            opencv::imgproc::resize_def(&frame, &mut disp_frm, Size::new(1920, 1080))?;
			highgui::imshow(window, &disp_frm)?;
		}
        let elapsed = loop_time.elapsed();
        let fps: f32 = 1.0 / (elapsed.as_secs_f32());
        ma.add_sample(fps);
        println!("FPS {}", ma.get_average());
        loop_time = Instant::now();
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
            if key == 'q' as i32 {
                break;
            }
            if key == 'f' as i32 {
                // this is a positive hit.
                // Save to positive/ folder.
                let filename = format!("positive/{}.jpg", elapsed.as_nanos());
                imwrite_def(&filename, &frame)?;
            }
            if key == 'd' as i32 {
                // this is a negative hit.
                // Save to negative/ folder.
                let filename = format!("negative/{}.jpg", elapsed.as_nanos());
                imwrite_def(&filename, &frame)?;
            }
		}
	}

    Ok(())
}

fn start_screencast() -> Result<ActiveScreenCast, Box<dyn std::error::Error>>{
    let screen_cast = ScreenCast::new()?.start(None)?;
    let fd = screen_cast.pipewire_fd();
    let streams: Vec<&ScreenCastStream> = screen_cast.streams().collect();
    let node_id: u32 = streams.first().unwrap().pipewire_node();
    println!("pipewire fd: {}", fd);
    println!("pipewire node_id: {}", node_id);
    //screen_cast.close()?;
    Ok(screen_cast)
}

fn video_capture(gst_string: String) -> Result<VideoCapture, Box<dyn std::error::Error>> {
    Ok(VideoCapture::from_file_def(&gst_string)?)
}
