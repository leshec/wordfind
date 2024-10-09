# wordfind
Web app: Crossword / scrabble word finder learning webproject

- Demo mode generates random tiles: then finds largest, highest score and valid words. 
- Search for words in a dictionary of 300k words
- Play a game to find words from random letters/tiles
- Find anagrams of words.

Shuttle is an awesome tool for quickly and easily deploying webapps based on Rust.
Tera for templating was good but I'll explore others
Axum is easy enough to use and I like the idea of HTMX

## **Website** 

- Wordfind (https://wordfinder.shuttleapp.rs/wordfind)


## **Todo:**

- [x] Favicon. What is the right way to do this? 
- [x] Experiment with loops in templates
- [x] Figure how to deploy with shuttle
- [ ] Make some notes for later
- [ ] error handling and testing etc


## **Project tools: HTMX, Tera, Axum, Missing and Shuttle**

- Web server: Rust with Axum (https://github.com/tokio-rs/axum) 
- HTML templating engine: Tera (https://docs.rs/tera/latest/tera)
- Basic styling: Missing.style (https://missing.style/)
- HTMX (https://htmx.org/)
- Tera (https://keats.github.io/tera/docs/#getting-started)
- Hotreloading: Cargo-watch (https://github.com/watchexec/cargo-watch)
- Hotreloading: Tower-livereload (https://github.com/leotaku/tower-livereload) - disabled
- Deployed via shuttle.rs on free tier (https://www.shuttle.rs/)


## **HTMX Example app:**

- Big Sky Software  (https://github.com/bigskysoftware/contact-app) 


## **Thanks to:**

- CodeScope Youtuber for inspiring me to have a go deploying a web app with his example of using HTMX and Rust. 
- Source code (https://gitlab.com/codescope-reference/rustmx)
- This site is based on Guardian coding challenges (https://github.com/guardian/coding-exercises)
- Note uses text file with approx 300k words. 
- Note an offical set of Scrabble words

## **Screenshot:**

![Here is an example screenshot](https://private-user-images.githubusercontent.com/103497129/375144670-6bd20ef7-42bb-4aee-aa0f-96dd2335620f.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3Mjg1MTE3NTUsIm5iZiI6MTcyODUxMTQ1NSwicGF0aCI6Ii8xMDM0OTcxMjkvMzc1MTQ0NjcwLTZiZDIwZWY3LTQyYmItNGFlZS1hYTBmLTk2ZGQyMzM1NjIwZi5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQxMDA5JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MTAwOVQyMjA0MTVaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT0wYzg3ZmQ1NWIwNWI0MzgzNjYyYTc1MDk3MTgyOTcxMDZlMzk0ODc5MGJiMjBmNjE1YmU1NWQ0NmNmMTE5ODllJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.LmIlohVqtCarxydCPvV8mjOKWA-Aw8HfYrTknTKmRPw)

## **Install:**

- Terminal `git clone https://github.com/leshec/wordfind.git`
- Need Rust installed, comes with Cargo package manager, from terminal `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install Shuttle or see their website e.g for linux/mac `curl -sSfL https://www.shuttle.rs/install | bash`
- A Windows version of Shuttle available
- Terminal `cargo build && cargo shuttle project run`
- Site runs locally on your machine e.g. via `127.0.0.1:8000`
- See docs on Shuttle.rs 
- Deplayment is pretty simple:
- Log in to shuttle via terminal `cargo shuttle login` note sign up is easy with Github, dashboard is minimal, prompts for API, takes you to website to get it, and then paste into terminal and run from command line `cargo shuttle deploy`
- Visit (https://www.shuttle.rs/) for more info, example and to deploy your own stuff loads of example


