        		novos
        ( aka 'novos omits very old standards' )	
            Copyright 2026 xer & contributors


		  WARNING!
		   novos is currently beta software. Expect frequent updates and potential breaking changes as we approach v1.0.
		   
		  What novos?
		  ----
		  novos is a lightning-fast static site generator (SSG) built in Rust. It takes your Markdown files and turns them into a website in milliseconds.


		  Why novos?
		  ----
	novos is a fast, all in one static site generator written in Rust.
	But, you might ask: Why novos?
	Well, why not? Actual answer this time:
	All in one, no hassling Node or Dart Sass again. Speed, it has a lot of it! Also, it has Tera (Jinja2, ever heard of it? kinda like that). And finally, a boat load of features:

	- Sass transpilation via native grass (no C++ or Node.js required)
	- Fast parallelism utilizing Rayon for multi-core page generation
	- Self-contained binary with embedded assets via rust-embed
	- Built-in RSS feed generation
	- Automated search.json generation for client-side search
	- Syntax highlighting powered by syntect
	- Automatic asset minification and WebP image conversion
	- Integrated Axum webserver with WebSocket support
	- Support for a wide variety of theme presets
	- Tera templating engine for powerful, reusable layouts


		Roadmap
		----

		[ ] Pagination 
	        [ ] Taxonomy
	        [X] Nordish theme
	        [X] Live reload in dev server
 

		Supported OSes
		----

		| Tier     | Operating System | Notes                    |
		|----------|---------------|-----------------------------|
		| Tier 1   | **OmniOS CE** | Primary development         |
		| Tier 1   | **FreeBSD**   | Primary development         |
		| Tier 2   | Ubuntu Noble  | Verified compatibility      |
		| Tier 2   | Arch Linux    | Verified compatibility      |
		| Tier 2   | Void Linux    | Verified compatibility      |
		| Tier 3   | macOS         | *"CI compiled it"*          |
		| Tier 3   | Windows       | *"Wine said it worked"*     |


		Themes
		----
		- Solarnight (themes/solarnight)
		  
		    - Solarnight is a Rose Pine inspired theme.
		    
		    
		    - Nordish (themes/nordish)
		      
		         - Nordish is, well.... a Nord inspired theme.
