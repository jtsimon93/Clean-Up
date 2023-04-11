#![windows_subsystem = "windows"]

use std::{
    env, fs, io,
    thread, thread::sleep,
    time::Duration,
    process::exit,
};
use walkdir::WalkDir;
use notify_rust::Notification;
use dirs;
use chrono::{DateTime, Local, Datelike, Timelike};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayEvent, TrayIconBuilder,
};
use winit::event_loop::{ControlFlow as WinitControlFlow, EventLoopBuilder};
use image;

fn main() {
    // Currently only targeting Windows, so let's ensure that no other OS is being used
    if env::consts::OS != "windows" {
        println!("This program is designed to work with Microsoft Windows. A different operating system was detected. Exiting.");
        Notification::new()
            .summary("Clean Up - Error")
            .body("Clean Up is designed to work with Microsoft Windows. A different operating system was detected. The program will now exit.")
            .timeout(0) // persistent notification
            .appname("Clean Up")
            .show()
            .unwrap();
        exit(1);
    }

    // Spawn a thread to run Clean Up
    thread::spawn(|| {
        run();
    });

    // Show the tray menu
    build_tray_menu();
}

fn build_tray_menu() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tray-icon.png");
    let icon = load_icon(std::path::Path::new(path));

    // Winit event loop
    let event_loop = EventLoopBuilder::new().build();

    // Tray menu
    let tray_menu = Menu::new();

    // Menu exit option
    let quit_i = MenuItem::new("Quit", true, None);

    // Add the exit option to the menu
    tray_menu.append_items(
        &[
            &quit_i,
        ]
    );

    // Display tray icon
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_tooltip("Clean Up")
            .with_menu(Box::new(tray_menu))
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayEvent::receiver();

    // Event loop for the tray icon
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = WinitControlFlow::Poll;


        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                *control_flow = WinitControlFlow::Exit;
            }
            println!("{event:?}");
        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    });
}

fn run() {
    let hour_to_delete_files = 12;
    let day_of_week_to_delete_files = chrono::Weekday::Sun;
    let temp_file_dir = get_home_directory() + "\\Documents\\temp";

    // Send notification to let the user know the software is running
    let running_notification_body = format!("Clean Up is running and will delete your temporary files at {}:00 every {}. The files inside your {} will be deleted.", hour_to_delete_files, day_of_week_to_delete_files.to_string(), temp_file_dir);
    Notification::new()
        .summary("Clean Up - Running")
        .body(&running_notification_body)
        .timeout(0) // persistent notification
        .appname("Clean Up")
        .show().unwrap();


    loop {
        let now: DateTime<Local> = Local::now();

        // Send a notification 15 minutes prior to deletion
        if now.weekday() == day_of_week_to_delete_files && now.hour() == (hour_to_delete_files - 1) && now.minute() == 45 {
            Notification::new()
                .summary("Clean Up - Deleting Temp Files in 15 Minutes")
                .body("Clean Up will delete your temp files in 15 minutes!")
                .appname("Clean Up")
                .show().unwrap();
        }

        // Delete temp files on the given day and time
        else if now.weekday() == day_of_week_to_delete_files && now.hour() == hour_to_delete_files {
            if WalkDir::new(&temp_file_dir).into_iter().count() > 0 {
                delete_temp_files(&temp_file_dir).unwrap();
                Notification::new()
                    .summary("Clean Up - Deleted Files")
                    .body("Your temp files were cleared")
                    .appname("Clean Up")
                    .show().unwrap();
            } else {
                Notification::new()
                    .summary("Clean Up - There were no files to delete")
                    .body("There were no temporary files to delete.")
                    .appname("Clean Up")
                    .show().unwrap();
            }
        }

        // Sleep for 1 minute
        sleep(Duration::from_secs(60));
    }
}

fn delete_temp_files(folder_path: &str) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn get_home_directory() -> String {
    if let Some(home_dir) = dirs::home_dir() {
        return home_dir.display().to_string();
    } else {
        Notification::new()
            .summary("Clean Up - Unable to find home directory")
            .body("Clean up was unable to determine your home directory.")
            .timeout(0)
            .appname("Clean Up")
            .show().unwrap();
        exit(1);
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}



