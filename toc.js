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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Rust Algorithm Club</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">💡 基礎概念</li><li class="chapter-item expanded "><a href="concepts/asymptotic-notation/index.html">漸進符號 Asymptotic Notation</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">🔍 搜尋</li><li class="chapter-item expanded "><a href="searching/linear_search/index.html">線性搜尋 Linear search</a></li><li class="chapter-item expanded "><a href="searching/binary_search/index.html">二元搜尋 Binary search</a></li><li class="chapter-item expanded "><a href="searching/interpolation_search/index.html">內插搜尋 Interpolation search</a></li><li class="chapter-item expanded "><a href="searching/exponential_search/index.html">指數搜尋 Exponential search</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">📚 排序</li><li class="chapter-item expanded affix "><li class="part-title">簡單排序</li><li class="chapter-item expanded "><a href="sorting/insertion_sort/index.html">插入排序 Insertion sort</a></li><li class="chapter-item expanded "><a href="sorting/selection_sort/index.html">選擇排序 Selection sort</a></li><li class="chapter-item expanded "><a href="sorting/bubble_sort/index.html">氣泡排序 Bubble sort</a></li><li class="chapter-item expanded "><a href="sorting/shellsort/index.html">希爾排序 Shellsort</a></li><li class="chapter-item expanded affix "><li class="part-title">高效排序</li><li class="chapter-item expanded "><a href="sorting/heapsort/index.html">堆積排序 Heapsort</a></li><li class="chapter-item expanded "><a href="sorting/quicksort/index.html">快速排序 Quicksort</a></li><li class="chapter-item expanded "><a href="sorting/mergesort/index.html">合併排序 Mergesort</a></li><li class="chapter-item expanded affix "><li class="part-title">混合排序</li><li class="chapter-item expanded "><div>🚧 內省排序 Introsort</div></li><li class="chapter-item expanded "><div>🚧 自適應合併排序 Timsort</div></li><li class="chapter-item expanded "><div>🚧 模式消除快速排序 Pdqsort</div></li><li class="chapter-item expanded affix "><li class="part-title">特殊排序</li><li class="chapter-item expanded "><a href="sorting/counting_sort/index.html">計數排序 Counting sort</a></li><li class="chapter-item expanded "><a href="sorting/bucket_sort/index.html">桶排序 Bucket sort</a></li><li class="chapter-item expanded "><a href="sorting/radix_sort/index.html">基數排序 Radix sort</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">🏠 資料結構</li><li class="chapter-item expanded affix "><li class="part-title">堆疊與佇列</li><li class="chapter-item expanded "><a href="collections/stack/index.html">堆疊 Stack</a></li><li class="chapter-item expanded "><a href="collections/queue/index.html">佇列 Queue</a></li><li class="chapter-item expanded "><a href="collections/deque/index.html">雙端佇列 Deque</a></li><li class="chapter-item expanded affix "><li class="part-title">鏈結串列</li><li class="chapter-item expanded "><a href="collections/linked_list/index.html">鏈結串列概述</a></li><li class="chapter-item expanded "><a href="collections/singly_linked_list/index.html">單向鏈結串列 Singly linked list</a></li><li class="chapter-item expanded "><div>🚧 雙向鏈結串列 Doubly linked list</div></li><li class="chapter-item expanded "><div>🚧 循環鏈結串列 Circular linked list</div></li><li class="chapter-item expanded affix "><li class="part-title">關聯容器</li><li class="chapter-item expanded "><a href="collections/associative-container/index.html">關聯容器概述</a></li><li class="chapter-item expanded "><a href="collections/hash_map/index.html">雜湊表 Hash map</a></li><li class="chapter-item expanded "><div>🚧 有序映射表 Ordered map</div></li><li class="chapter-item expanded "><div>🚧 多重映射表 Multimap</div></li><li class="chapter-item expanded "><a href="collections/set/index.html">集合 Set</a></li><li class="chapter-item expanded "><a href="collections/bloom_filter/index.html">布隆過濾器 Bloom filter</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">🧵 字串處理</li><li class="chapter-item expanded "><a href="hamming_distance/index.html">漢明距離 Hamming distance</a></li><li class="chapter-item expanded "><a href="levenshtein_distance/index.html">萊文斯坦距離 Levenshtein distance</a></li><li class="chapter-item expanded "><div>🚧 最長共同子字串 Longest common substring</div></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="CONTRIBUTING.html">貢獻指南</a></li><li class="chapter-item expanded affix "><a href="404.html">404</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
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
