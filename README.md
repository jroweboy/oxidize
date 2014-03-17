![logo](https://raw.github.com/jroweboy/oxidize/master/oxidize_logo.png)

A very basic web framework for Rust based off ideas from lots of other web frameworks.

Goals
======

oxidize aims to make it easy to write clean and testable web application that is both fast and safe. Rust as a language has amazing support of many of the things that you would want from a web framework but surprisingly there isn't any rich frameworks yet! oxidize will fill that hole by providing a clean framework that should be simple to use, sufficiently decoupled (able to swap components without having to hack at the core), and fast (just like the goal of rust). As such, a primary milestone of oxidize will be to enter the framework into the web framework benchmarks by TechEmpower.

Rust Verison
=======

oxidize tries to keep up with rust master as much as possible, but it is hard to constantly update all of the dependancies, their test cases, oxidize and the example applications. Because of this, for now, the README will contain the latest know hash that it will guarenteed compile for and so if it just so happens to not compile on master, you can at least load that version and it will compile. Once I update it to a rust master I will post a commit hash into the README (it is currently not compiling as of right now). 

Building Oxidize
=======

If you don't currently have the latest rust, you'll need to update to the version listed above. If you are on a debian/ubuntu distro you can run the following commands to add a rust-nightly repo (but be warned that oxidize might not compile on the nightlies until it catches up) 

    sudo add-apt-repository ppa:hansjorg/rust
    sudo apt-get update
    sudo apt-get install rust-nightly

Of course if you want to build rust from source, I recommend that you use the clone to the commit hash listed above. After you have a working rustc on your machine it should be simple to compile. You will also need libpcre in order to compile rust-prce so install it through your systems package manager. 

    # get libpcre -- this is an example command for ubuntu
    sudo apt-get install libpcre3-dev

    # get oxidize and build it
    git clone --recursive https://github.com/jroweboy/oxidize.git
    cd oxidize
    make

Some other useful make commands you should know are `make clean-all` which will call make clean on the dependancies as well. Currently due to my poor make file skills, if you need to rebuild a dependancy you will need to rebuild all of them with `make clean-all`. Another useful command is `make run` will run the example hello_world. 

Contributing
============

Probably the best way to contribute is to offer feedback about what you like and dislike about the framework in general. We wanna make a framework that people will enjoy coding in with all the other awesome guarentees that Rust brings. 

Another good way to contribute is by forking it and sending a pull request. There are tons of TODOs scattered throughout the codebase and each one represents an action item I would like to fix. Feel free to take one and fix it up. If it influences a fundamental part of the public API though I hope that you will take the time to explain why you think that it will need to change. Additionally, writing a new example that showcases the feature would be a really cool thing for you to do. 
Lastly, we need lots of test cases and we also need someone that is good at rust to help me find places to make the code more idiomatic :) Sometimes, I have no clue why I wrote the code I did besides the fact that it made the code compile. If there are some oddities in the way I handle lifetimes or pointers please let me know.

Authors
=======

 James Rowe - (jroweboy)

 Robert Hickman - (robhobbes)