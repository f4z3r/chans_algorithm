# chans_algorithm

### How to run it
Simply pull from git and run `cargo run <point number> <thread number>` from project root directory. If performance is of importance, make sure to build with  `--release` flag before running.

#### Animation
If you want the program to output extra files to use for animation, use the flag `--feature="animate"` when running. This will create a separate `hull_{}.csv` file for each thread and print graham's scan steps to the log instead of printing system report to it. However, note that to use the steps for graham's scan, it is only possible when running on a single thread as otherwise all data from all threads will be collected in the log.

### Notes
Note that any csv files in the `static` directory is ignored as it will contain files of generated points. These files can become very large when running with millions of points and are then not supported by Github.
