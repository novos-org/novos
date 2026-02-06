# Shortcodes
Shortcodes are reusable HTML components stored in your includes/shortcodes/ directory. They allow you to inject complex UI elements into Markdown and templates without writing inline HTML.

# Storage & Naming
- **Path**: includes/shortcodes/*.html
- **Naming**: The filename (minus the .html extension) becomes the command name.
    - **Example**: includes/shortcodes/notice.html is called via <% .notice %>.

# Invocation Syntax
Shortcodes use a dot-prefix to distinguish them from standard variables or includes.
```html
{% .shortcode_name "arg1" "arg2" "arg3" %}
```

> [!TIP]
> Arguments with spaces: Wrap them in double quotes. Simple arguments: Quotes are optional if there are no spaces (e.g., `{% .youtube dQw4w9WgXcQ %}`).

# Internal Logic
Inside the .html file, arguments are accessed via indexed variables starting at 1.

| Variable | Usage |
|---|---|
| `<%% 1 %%>` | The first argument passed. |
| `<%% 2 %%>` | The second argument passed. |

And so on.
