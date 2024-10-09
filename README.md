# wordfind
Web app: Toy scrabble word finder

Generates random tiles: then finds largest, highest score and valid words. 


## **Website** 

- Wordfind (https://wordfinder.shuttleapp.rs/wordfind)


## **Todo:**

- [x] Favicon. What is the right way to do this? 
- [x] Experiment with loops in templates
- [x] Figure how to deploy with shuttle
- [x] Use it to port some of my projects e.g. this one scrabble
- [ ] Make some notes for later
- [ ] maybe add styling in the layout rather than inline. Ugh no 
- [ ] error handling and testing etc
- [ ] add some post requests e.g. add in your own letters
- [ ] gamify it with option to find as many words as you can with a given random set


## **Project: HTMX, Tera, Axum, Missing and Shuttle**

- Web server: Rust with Axum (https://github.com/tokio-rs/axum) 
- HTML templating engine: Tera (https://docs.rs/tera/latest/tera)
- Basic styling: Missing.style (https://missing.style/)
- HTMX (https://htmx.org/)
- Tera (https://keats.github.io/tera/docs/#getting-started)
- Hotreloading: Cargo-watch (https://github.com/watchexec/cargo-watch)
- Hotreloading: Tower-livereload (https://github.com/leotaku/tower-livereload)
- Deployed via shuttle.rs on free tier (https://www.shuttle.rs/)


## **HTMX Example app:**

- Big Sky Software  (https://github.com/bigskysoftware/contact-app) 


## **Thanks to:**

- CodeScope Youtuber for getting me started with HTMX and rust. 
- Source code (https://gitlab.com/codescope-reference/rustmx)
- Based on Guardian coding challenges (https://github.com/guardian/coding-exercises)
- Note uses text file with approx 179k words. Less than official dictionary for Scrabble

## **Screenshot:**


![Letters: D,I,K,N,O,Q,R Returns: longest word, "drink", highest, "Qi = 10"](https://github.com/user-attachments/assets/72c323ea-6902-4a4d-8091-a1b6b104c4a4)


## **Install:**

- if you want to take this and turn it into something else...and notes to self
- make sure you have rust installed. It comes with cargo the package manager
- see one-liner on (https://www.rust-lang.org/) install page: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- clone this project: from terminal run `git clone https://github.com/leshec/wordfind.git`
- go to the cloned repo `cd wordfind`
- from terminal run `cargo install cargo-shuttle` so your system has the shuttle stuff
- from terminal run `cargo shuttle run` - may need a login api key via shuttle.rs. Just sign up to shuttle and it gives you an api key
- your system may do a pop up asking for permissions to access the network
- should be able to see the site running locally on your machine e.g. via `127.0.0.1:8000`
- visit (https://www.shuttle.rs/) for more info, example and to deploy your own stuff


