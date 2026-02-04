# Shortcodes - docs

Shortcodes live in includes/shortcodes/*.html

They are invoked by `<% .<name> <args> %>`

Example shortcode:

```html
<div class="video-container">
    <iframe 
        style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: 0;"
        src="https://www.youtube.com/embed/<%= a1 =%>" 
        title="YouTube video player" 
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        allowfullscreen>
    </iframe>
</div>
```