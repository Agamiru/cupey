# cupey

```cupey``` is a simple command line application for recursively copying files from an originating folder
into your current directory (i.e. the directory from which you call the ```cupey``` command).

** Note: Cupey will copy all the *contents* of the directory given, it won't copy the directory itself.


# Example
```bash
cupey --from "C:\users\my_user\Desktop\rust projects"
```
This will copy all the files (and folders) from 'rust projects' into your present working directory.
This won't overwrite any existing file in the current directory that shares the same name with any
file being copied, Cupey will skip those files.

On a windows terminal, the above command could take this form:
```cmd
C:\users\my_user\python_projects>cupey -f "C:\users\my_user\Desktop\rust projects" --overwrite
```
This will copy all files from 'Desktop\rust projects' into 'my_user\python_projects' and overwrite
all existing files in 'python projects' with same name with those from 'rust_projects'.

# How To Test
By default, Rust runs tests concurrently on different threads, some of the tests in this project are
interdependent so use the ```--test-threads=1``` flag to run tests synchronoulsy.

```bash
cargo test -- --test-threads=1
```

# But Why?
I needed a tool to 'cupey' some of my Flutter widgets from a folder where I store reusable widgets
into the folder I needed to use them in. A command line tool would keep me from switching windows when I
need to perform this task, that's why I built it.

# Todo
- Add skip feature: Skip some files during copying.
- Add select feature: Copy only selected files.
- Add color to stdout error messages.
