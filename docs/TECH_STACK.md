# TECH STACK

During initial prototyping, i'd like to build up the full backend in Rust and run it on my desktop. The frontend can then be built for multiple clients, but mobile is the primary so we'll start with that. I'd like to write the backend myself and learn what needs to be learned. I may also attempt to utilize some half AI half human workflows that I've seen around on twitter and such.

This will be a Swift iPhone app. I don't know Swift so I'll fully vibecode the frontend. I'll buy an apple developer account for this.

Database should be postgres (I'll use my supabase account because I can easily start free). Backend should be a clean and intuitive API for the client to grab data to be easily displayed. An AI-optimized MCP or CLI can be built on top of these APIs.

Audio should be built up in a buffer (sent over some kind of persistent connection from frontend to backend). Once buffer is large enough, use speech to text and put text in buffer. Once text is large enough, use jev to classify it to say if it is ready for LLM interpretation. If it is, use jev to determine how many of the buffer tokens (words) should be sent to AI and how many should be left for the next buffer. Send the words to small LLM (start with Luna none or Muse Ember) to rewrite in the minimal and simple style and also determine calories and protein.

Also: I plan to use this app myself for a while before ever attempting to share and scale it. For this reason, I won't even incorporate auth or extra stuff like privacy policy and stuff yet.
