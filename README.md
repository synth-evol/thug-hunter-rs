# Thug Hunter RS
'Thug Hunter' is the silly name for this silly project. Its expressed purpose is to provide a quick way to see how much a given osu! user is deranking by comparing their 'duel rank' with their global rank, and giving them a derank 'rating' in the form of a percentage. 

## Download and Run Thug-Hunter-RS locally
You will need a working Rust and Cargo installation for this.

Instructions are written for Linux, though the same commands should work on macOS to my knowledge. If you're on Windows, you'll need Git Bash anyway to clone the repo so they should work there too. 

clone the repo with SSH (Github has good guides for how to set this up if you haven't already):
`git clone git@github.com:synth-evol/thug-hunter-rs.git`

Navigate to the repo folder
`cd thug-hunter-rs`

Build the project:
`cargo build`

Run the project:
`cargo run -- <OSU_API_KEY>`
Replace OSU_API_KEY with your v1 osu api key. Instructions for getting an API key can be found on the Osu API git repo: [Osu API v1](https://github.com/ppy/osu-api/wiki#requesting-access)

The project will boot up on your localhost on port 3000. To get the global score of a given user, navigate to `localhost:3000/<username>` in your browser, with <username> replaced with the username of whatever player you're interested in seeing the stats of. (Right now this just returns the user's global rank. Eventually should return duel rank, global rank, and derank %)