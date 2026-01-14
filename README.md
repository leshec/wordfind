# wordfinder

Trying out deployment via Railway and Leapcell.

Railway: EU deployed 30-60ms for me (I'm europe)
https://wordfind-production.up.railway.app/

Leapcell: slower e.g. 150-200ms US west or Tokyo servers.
https://wordfind-leshec290-i6w2gi13.leapcell.dev/

Web app: Crossword / scrabble word finder learning webproject

- Demo mode generates random tiles: then finds largest, highest score and valid words
- Search for words in a dictionary of 300k words
- Play a game to find words from random letters/tiles
- Find anagrams of words

Not 100% finished but functions enough as intended

## **Website** 

- Wordfinder (https://wordfinder.shuttleapp.rs)


## **Todo:**

- [x] Favicon. What is the right way to do this? 
- [x] Experiment with loops in templates
- [x] Figure how to deploy with shuttle. Edit Shuttle closed down. 
- [ ] Make some notes for later
- [ ] error handling and testing etc
- [x] add an about and contact for new words

## **For another day...**
- [ ] add a high score leaderboard for the games
- [ ] add a pop over if you click on a word to show meaning of word, there is a nice dictionary API for that


## **Project tools: HTMX, Tera, Axum, Missing and shuttle**

- Web server: Rust with Axum (https://github.com/tokio-rs/axum) 
- HTML templating engine: Tera (https://docs.rs/tera/latest/tera)
- Basic styling: Missing.style (https://missing.style/)
- HTMX (https://htmx.org/)
- Tera (https://keats.github.io/tera/docs/#getting-started)
- Hotreloading: Cargo-watch (https://github.com/watchexec/cargo-watch)
- Hotreloading: Tower-livereload (https://github.com/leotaku/tower-livereload) - disabled
- ~~Deployed via shuttle.rs on free tier (https://www.shuttle.rs/). Edit: closed/deprecated.


## **HTMX Example app:**

- Big Sky Software  (https://github.com/bigskysoftware/contact-app) 

## **Thanks to:**

- CodeScope Youtuber for inspiring me to have a go deploying a web app with his example of using HTMX and Rust. 
- Source code (https://gitlab.com/codescope-reference/rustmx)
- This site is based on Guardian coding challenges (https://github.com/guardian/coding-exercises)
- Note uses text file with approx 300k words
- Not an offical set of Scrabble words or authority on valids words

## **Screenshot:**

![screen_shot_wordfind](https://github.com/user-attachments/assets/f6cb1c9c-fd35-4ebe-b64e-ac0d7fff6a6a)

## **Install:**

If you want to mess around with it...

- Terminal `git clone https://github.com/leshec/wordfind.git`
- Need Rust installed, comes with Cargo package manager, from terminal `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Terminal `cargo build` 
- Site runs locally on your machine e.g. via `127.0.0.1:8080`


