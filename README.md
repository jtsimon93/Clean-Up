# Clean Up
Clean Up is a program for Windows that automatically deletes all of the files inside a set folder every week on a given day and time. 

By default, the program deletes all files inside the temp folder in your Documents folder at 12:00PM on Sundays. 

Inside of the `run()` function, you can set the day, time, and folder to delete files within. 

```rust
    let hour_to_delete_files = 12;
    let day_of_week_to_delete_files = chrono::Weekday::Sun;
    let temp_file_dir = get_home_directory() + "\\Documents\\temp";
```