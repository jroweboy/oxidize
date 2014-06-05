![oxidize logo](https://raw.github.com/jroweboy/oxidize/master/oxidize_logo_smaller.png)

A basic web framework for Rust. Subject to be redesigned when issues arise (and when Rust updates)

Goals
=====

Make a small, extensible micro framework to allow users the freedom to choose whatever external libraries they want 

Rust Verison
============

I try to keep it up to date with master, but if it ever falls behind, it should be too hard to update. The codebase is pretty small

Building Oxidize
================

    git clone --recursive https://github.com/jroweboy/oxidize
    cd oxidize && ./build-deps.sh

oxidize is now using rust-empty to provide the makefile, so you will need to follow those guidelines. The only dependency for oxidize is rust-http, so before you build oxidize, you will need to first build rust-http. Then you will need to symlink the libraries into the target directory. This target directory is a little weird right now since it is trying to match up to what cargo expects. Hopefully this will become much less painful someday when cargo gets completed. Till then I made a short command list filled with the commands to run as a bash script (aka the build deps script). If this fails though, you will want to do those steps that I just mentioned and maybe try each command in build_deps one at a time.

Other projects you may want as dependancies for your project

 * rust-postgres
 * rust-mustache
 * jinja2-c (my attempt at making c/rust bindings for jinja2)
 * rust-http (I should really expose this whole thing through oxidize since it's darn useful)


Contributing
============

Probably the best way to contribute is to offer feedback about what you like and dislike about the framework in general. We wanna make a framework that people will enjoy coding in with all the other awesome guarentees that Rust brings. 

A very cool way to contribute would be to try to make something in oxidize and then tell me what you liked and what you disliked. I read lots and lots of blog posts about what different people like in every sort of web framework, and I usually take the things that I like from there. But as of yet, no one has made anything substantial in oxidize yet so I don't know where to improve. Over the summer, I will make something big in oxidize so expect big things from that as well.

Authors
=======

If you've ever commited code to this repository, feel free to add your name here!

 James Rowe - (jroweboy)

 Robert Hickman - (robhobbes)
