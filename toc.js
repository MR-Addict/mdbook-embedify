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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">Basics</li><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> Intro</a></li><li class="chapter-item expanded "><a href="usage.html"><strong aria-hidden="true">2.</strong> Usage</a></li><li class="chapter-item expanded "><a href="more-apps.html"><strong aria-hidden="true">3.</strong> More Apps</a></li><li class="chapter-item expanded "><a href="ignore-embeds.html"><strong aria-hidden="true">4.</strong> Ignore Embeds</a></li><li class="chapter-item expanded "><a href="global-embedding.html"><strong aria-hidden="true">5.</strong> Global Embedding</a></li><li class="chapter-item expanded affix "><li class="part-title">Apps</li><li class="chapter-item expanded "><a href="third-party/index.html"><strong aria-hidden="true">6.</strong> Third Party Apps</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="third-party/gist.html"><strong aria-hidden="true">6.1.</strong> Gist</a></li><li class="chapter-item "><a href="third-party/vimeo.html"><strong aria-hidden="true">6.2.</strong> Vimeo</a></li><li class="chapter-item "><a href="third-party/giscus.html"><strong aria-hidden="true">6.3.</strong> Giscus</a></li><li class="chapter-item "><a href="third-party/youtube.html"><strong aria-hidden="true">6.4.</strong> Youtube</a></li><li class="chapter-item "><a href="third-party/codepen.html"><strong aria-hidden="true">6.5.</strong> Codepen</a></li><li class="chapter-item "><a href="third-party/stackblitz.html"><strong aria-hidden="true">6.6.</strong> Stackblitz</a></li><li class="chapter-item "><a href="third-party/codesandbox.html"><strong aria-hidden="true">6.7.</strong> Codesandbox</a></li><li class="chapter-item "><a href="third-party/bilibili.html"><strong aria-hidden="true">6.8.</strong> Bilibili</a></li></ol></li><li class="chapter-item expanded "><a href="local/index.html"><strong aria-hidden="true">7.</strong> Local Apps</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="local/footer.html"><strong aria-hidden="true">7.1.</strong> Footer</a></li><li class="chapter-item "><a href="local/include.html"><strong aria-hidden="true">7.2.</strong> Include</a></li><li class="chapter-item "><a href="local/scroll-to-top.html"><strong aria-hidden="true">7.3.</strong> Scroll to Top</a></li><li class="chapter-item "><a href="local/announcement-banner.html"><strong aria-hidden="true">7.4.</strong> Announcement Banner</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
