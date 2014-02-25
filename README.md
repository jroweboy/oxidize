oxidize
=======

A very basic web framework for Rust based off lots of other web frameworks.

Build
=======

oxidize builds in Rust nightly. if you don't currently have the latest rust, you'll need to update before installing. Building oxidize should be very simple. Just clone --recursive to get oxidize and all of its dependancies and the run make. The make command should recurse into each of the libraries and build them. One of the dependancies is libpcre, so if you don't have it you can usually find it by using your systems package manager to install it. Some other useful make commands you should know are make clean-all which will call make clean on the dependancies as well (if you ever need to rebuild the dependancies this is the most reliable way sadly.) Also make run will run the example hello_world. 

Contributing
============

Probably the best way to contribute is to offer feedback about what you like and dislike about the framework in general. We wanna make a framework that people will enjoy coding in with all the other awesome guarentees that Rust brings. 

Another good way to contribute is by forking it and sending a pull request. There are tons of TODOs scattered throughout the codebase and each one represents an action item I would like to fix. Feel free to take one and fix it up. If it influences a fundamental part of the public API though I hope that you will take the time to explain why you think that it will need to change. Additionally, writing a new example that showcases the feature would be a really cool thing for you to do. 
Lastly, we need lots of test cases and we also need someone that is good at rust to help me find places to make the code more idiomatic :) Sometimes, I have no clue why I wrote the code I did besides the fact that it made the code compile. If there are some oddities in the way I handle lifetimes or pointers please let me know.

Authors
=======

James Rowe - (jroweboy)
Robert Hickman - (robhobbes)