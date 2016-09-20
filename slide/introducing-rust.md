# Introduction to Rust

## Adityo Pratomo (@kotakmakan)

---

# This Talk is Also Available at

github.com/lunchboxav/rust-intro/webserver

---

# Background

- Chief Academic Officer at Framework
- Chief Technology Officer at Labtek Indie

---

# Framework

- Providing software development course, training and workshop
- Based in BSD

![](https://scontent.cdninstagram.com/t51.2885-19/s150x150/12501964_776556112474284_1583292585_a.jpg)

---

# Labtek Indie

- Rapid Prototyping As A Service
- Based in Bandung

![](https://d1qb2nb5cznatu.cloudfront.net/startups/i/321497-a83fd6324ed6c76e05dc137012e59bef-medium_jpg.jpg?buster=1389436901)

---

# Codewise, I'm

- Generalist
- Creative Coder
- C/C++, Java, JS

---

# Me and C++

- Met during undergrad (2005)
- Professionally using it since 2012
- Tool: openFrameworks, Cinder and Unreal Engine

---

- Interactive installation for Nike (2012)

![](http://www.adityo.net/wp-content/uploads/2012/08/DSC_0008-e1346043109148.jpg)

---

- VR work (2015)

![](http://www.adityo.net/wp-content/uploads/2015/07/Screen-Shot-99-Names-600x353.png)

---

- Recent work: Interactive Video and Sound for Australia National Maritime Museum (2015)
- Coded in C++, running in Raspberry Pi
![](http://www.actionstations.sydney/assets/Uploads/Gallery/_resampled/CroppedImage1130635-ImmersiveCinema-1130x636.jpg)

---

# C++ for Me

- (+) Fast product
- (+) Runs on many platforms
- (+) Robust debugging tool
- (-) Still have issues with pointers
- (-) Segfaults here and there

---

# When I Meet Rust

- System programming language
- Like C++, without segfaults (yum!)
- Better handling of reference and pointers
- Mixture of imperative and functional paradigm

---

# Why Rust?

- Easier to write secure code
- Easier to write multithreaded code (hello concurrency)
- Runs on many devices/platforms

---

# What to Do with Rust?

- System programming
- Something low-level enough to benefit from precise memory management
    - Web Browser (Mozilla Firefox)
    - Distributed storage system (Dropbox)
    - 3D Games
    - Device drive
    - Operating System
- General tool

---

# Rust Compared to Other

- Direct alternative to C++
- Lower level access to Go/Python
- Not just for network application like Node.js

---

# Rust's Killer Features

- Type safety
- Traits based generics
- Pattern matching
- Memory safety

---

#  How Rust is Fast and Safe?

- Extensive compiler checking
- Fast: No garbage collection, Rust automatically detect when to free memory
    + Ownership of data 
- Safe: No data race, guaranteed data lifetime, no dangling-pointer

--- 

# Why Learn New Language?

- Gains new perspective on how things are done
- Gains new understanding on programming itself
- Make old and new things in a different way

---

# Learning Rust by Doing

- Context for personal learning -> doing something familiar enough
- Learn guitar by playing song, not by memorizing chords
- We'll try exercises provided in [Rustlings](https://github.com/carols10cents/rustlings)

---

# Exploring Rust

- Rust has bunch of packages and libraries, called Crates
- Available at [crates.io](http://www.crates.io)
- Let's make a simple web service using Iron

---

# Iron

- A web server framework for Rust
- Concurrent by default
- Includes additional middleware (ex: Router)
- Based on hyper, an HTTP implementation for Rust

---

# Using Iron

`Cargo.toml`

```
[dependencies]
iron = "*"
```


---

# Using Iron 

`main.src`

```
extern crate iron;

use iron::prelude::*;
use iron::status;
```

---

# A Simple Web Service

[github.com/lunchboxav/rust-intro/webserver](github.com/lunchboxav/rust-intro/webserver)

---

# Tips for Learning Rust

- Katas: learn by making familiar things
- Try make small tool to replace your existing tool
- Consult the documentation
- Ask people on SO/Twitter
- Organize a community

---

# Learning Resources

- The Rust Book (https://doc.rust-lang.org/book/)
- Rust 101 (https://www.ralfj.de/projects/rust-101/main.html)
- Rust Tutorial (http://aml3.github.io/RustTutorial/html/toc.html)
- Rust Syntax (https://gist.github.com/brson/9dec4195a88066fa42e6)
- Rust By Example (http://rustbyexample.com/expression.html)
- Rustlings, small Rust Exercises (https://github.com/carols10cents/rustlings)
- 24 Days of Rust (http://zsiciarz.github.io/24daysofrust/)
- Rust FFI Omnibus (http://jakegoulding.com/rust-ffi-omnibus/)
- New Rustacean (http://www.newrustacean.com)
