// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">2.1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="setup.html"><strong aria-hidden="true">2.2.</strong> Setup</a></li><li class="chapter-item expanded "><a href="basic-usage.html"><strong aria-hidden="true">2.3.</strong> Basic Usage</a></li></ol></li><li class="chapter-item expanded "><a href="core-features.html"><strong aria-hidden="true">3.</strong> Core Features</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="piping-context.html"><strong aria-hidden="true">3.1.</strong> Piping and Context</a></li><li class="chapter-item expanded "><a href="session-management.html"><strong aria-hidden="true">3.2.</strong> Session Management</a></li><li class="chapter-item expanded "><a href="web-search.html"><strong aria-hidden="true">3.3.</strong> Web Search</a></li></ol></li><li class="chapter-item expanded "><a href="subcommands.html"><strong aria-hidden="true">4.</strong> Subcommands</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="view-command.html"><strong aria-hidden="true">4.1.</strong> View Command</a></li><li class="chapter-item expanded "><a href="config-command.html"><strong aria-hidden="true">4.2.</strong> Config Command</a></li><li class="chapter-item expanded "><a href="session-command.html"><strong aria-hidden="true">4.3.</strong> Session Command</a></li><li class="chapter-item expanded "><a href="image-command.html"><strong aria-hidden="true">4.4.</strong> Image Command</a></li><li class="chapter-item expanded "><a href="tts-command.html"><strong aria-hidden="true">4.5.</strong> TTS Command</a></li><li class="chapter-item expanded "><a href="embedding-command.html"><strong aria-hidden="true">4.6.</strong> Embedding Command</a></li><li class="chapter-item expanded "><a href="agent-command.html"><strong aria-hidden="true">4.7.</strong> Agent Command</a></li><li class="chapter-item expanded "><a href="upgrade-command.html"><strong aria-hidden="true">4.8.</strong> Upgrade Command</a></li></ol></li><li class="chapter-item expanded "><a href="configuration.html"><strong aria-hidden="true">5.</strong> Configuration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="environment-variables.html"><strong aria-hidden="true">5.1.</strong> Environment Variables</a></li><li class="chapter-item expanded "><a href="custom-api-endpoints.html"><strong aria-hidden="true">5.2.</strong> Custom API Endpoints</a></li></ol></li><li class="chapter-item expanded "><a href="advanced-usage.html"><strong aria-hidden="true">6.</strong> Advanced Usage</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="file-input.html"><strong aria-hidden="true">6.1.</strong> File Input</a></li><li class="chapter-item expanded "><a href="model-selection.html"><strong aria-hidden="true">6.2.</strong> Model Selection</a></li><li class="chapter-item expanded "><a href="system-prompts.html"><strong aria-hidden="true">6.3.</strong> System Prompts</a></li></ol></li><li class="chapter-item expanded "><a href="examples.html"><strong aria-hidden="true">7.</strong> Examples</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="text-processing.html"><strong aria-hidden="true">7.1.</strong> Text Processing</a></li><li class="chapter-item expanded "><a href="image-analysis.html"><strong aria-hidden="true">7.2.</strong> Image Analysis</a></li><li class="chapter-item expanded "><a href="voice-synthesis.html"><strong aria-hidden="true">7.3.</strong> Voice Synthesis</a></li></ol></li><li class="chapter-item expanded "><a href="troubleshooting.html"><strong aria-hidden="true">8.</strong> Troubleshooting</a></li><li class="chapter-item expanded "><a href="development-workflow.html"><strong aria-hidden="true">9.</strong> Development</a></li><li class="chapter-item expanded "><a href="contributing.html"><strong aria-hidden="true">10.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="reference.html"><strong aria-hidden="true">11.</strong> Reference</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cli-options.html"><strong aria-hidden="true">11.1.</strong> Command Line Options</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
