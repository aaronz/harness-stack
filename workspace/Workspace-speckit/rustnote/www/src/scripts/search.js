class SearchManager {
    constructor() {
        this.dialog = null;
        this.query = '';
        this.replaceText = '';
        this.results = [];
        this.currentIndex = -1;
    }
    
    show() {
        if (this.dialog) return;
        
        this.dialog = document.createElement('div');
        this.dialog.id = 'search-dialog';
        this.dialog.innerHTML = `
            <div class="search-row">
                <input type="text" id="search-input" placeholder="Find...">
                <button id="search-next" title="Next">↓</button>
                <button id="search-prev" title="Previous">↑</button>
            </div>
            <div class="search-row">
                <input type="text" id="replace-input" placeholder="Replace...">
                <button id="replace-one" title="Replace">Replace</button>
                <button id="replace-all" title="Replace All">All</button>
            </div>
            <div class="search-row search-count" id="search-count"></div>
            <button id="search-close" class="search-close">✕</button>
        `;
        
        document.getElementById('editor').appendChild(this.dialog);
        
        this.dialog.querySelector('#search-close').addEventListener('click', () => this.hide());
        this.dialog.querySelector('#search-next').addEventListener('click', () => this.findNext());
        this.dialog.querySelector('#search-prev').addEventListener('click', () => this.findPrev());
        this.dialog.querySelector('#replace-one').addEventListener('click', () => this.replaceOne());
        this.dialog.querySelector('#replace-all').addEventListener('click', () => this.replaceAll());
        
        this.dialog.querySelector('#search-input').addEventListener('input', (e) => {
            this.query = e.target.value;
            this.findAll();
        });
        
        this.dialog.querySelector('#replace-input').addEventListener('input', (e) => {
            this.replaceText = e.target.value;
        });
        
        this.dialog.querySelector('#search-input').addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.findNext();
            }
        });
        
        this.dialog.querySelector('#search-input').focus();
    }
    
    findAll() {
        this.clearHighlights();
        
        if (!this.query) {
            this.results = [];
            this.currentIndex = -1;
            this.updateCount();
            return;
        }
        
        const content = editor.getContent();
        try {
            const regex = new RegExp(this.escapeRegex(this.query), 'gi');
            this.results = [];
            
            let match;
            while ((match = regex.exec(content)) !== null) {
                this.results.push({
                    index: match.index,
                    length: match[0].length,
                    text: match[0]
                });
            }
            
            this.currentIndex = this.results.length > 0 ? 0 : -1;
            this.highlightAll();
            this.highlightCurrent();
            this.updateCount();
        } catch (e) {
            this.results = [];
            this.updateCount();
        }
    }
    
    escapeRegex(string) {
        return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }
    
    highlightAll() {
        const container = editor.container;
        if (!container || this.results.length === 0) return;
        
        this.results.forEach((result, idx) => {
            this.highlightRange(result, idx === this.currentIndex);
        });
    }
    
    highlightRange(result, isCurrent) {
        const container = editor.container;
        if (!container) return;
        
        const range = document.createRange();
        const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, null, false);
        
        let charCount = 0;
        let startNode = null;
        let startOffset = 0;
        let endNode = null;
        let endOffset = 0;
        
        while (walker.nextNode()) {
            const node = walker.currentNode;
            const nextCount = charCount + node.length;
            
            if (charCount <= result.index && result.index < nextCount) {
                startNode = node;
                startOffset = result.index - charCount;
            }
            
            if (charCount < result.index + result.length && result.index + result.length <= nextCount) {
                endNode = node;
                endOffset = result.index + result.length - charCount;
                break;
            }
            
            charCount = nextCount;
        }
        
        if (startNode && endNode) {
            try {
                range.setStart(startNode, startOffset);
                range.setEnd(endNode, endOffset);
                
                const span = document.createElement('span');
                span.className = isCurrent ? 'search-highlight search-highlight-current' : 'search-highlight';
                range.surroundContents(span);
            } catch (e) {
                // Range may be invalid due to node structure
            }
        }
    }
    
    highlightCurrent() {
        const container = editor.container;
        if (!container) return;
        
        container.querySelectorAll('.search-highlight-current').forEach(el => {
            el.classList.remove('search-highlight-current');
        });
        
        if (this.results.length === 0 || this.currentIndex < 0) return;
        
        const result = this.results[this.currentIndex];
        editor.scrollToOffset(result.index);
        
        const allHighlights = container.querySelectorAll('.search-highlight');
        if (allHighlights.length > this.currentIndex) {
            allHighlights[this.currentIndex].classList.add('search-highlight-current');
        }
    }
    
    clearHighlights() {
        const container = editor.container;
        if (!container) return;
        
        const highlights = container.querySelectorAll('.search-highlight');
        highlights.forEach(highlight => {
            const parent = highlight.parentNode;
            while (highlight.firstChild) {
                parent.insertBefore(highlight.firstChild, highlight);
            }
            parent.removeChild(highlight);
        });
        parent.normalize?.();
    }
    
    updateCount() {
        const countEl = document.getElementById('search-count');
        if (!countEl) return;
        
        if (this.results.length === 0) {
            countEl.textContent = this.query ? 'No matches' : '';
        } else {
            countEl.textContent = `${this.currentIndex + 1} of ${this.results.length}`;
        }
    }
    
    findNext() {
        if (this.results.length === 0) return;
        this.currentIndex = (this.currentIndex + 1) % this.results.length;
        this.highlightCurrent();
        this.updateCount();
    }
    
    findPrev() {
        if (this.results.length === 0) return;
        this.currentIndex = (this.currentIndex - 1 + this.results.length) % this.results.length;
        this.highlightCurrent();
        this.updateCount();
    }
    
    replaceOne() {
        if (this.results.length === 0 || this.currentIndex < 0) return;
        
        const result = this.results[this.currentIndex];
        const content = editor.getContent();
        
        const newContent = content.substring(0, result.index) + 
                          this.replaceText + 
                          content.substring(result.index + result.length);
        
        editor.setContent(newContent);
        
        this.findAll();
        
        if (currentDocument) {
            currentDocument.isDirty = true;
            updateTitle();
        }
    }
    
    replaceAll() {
        if (this.results.length === 0) return;
        
        const content = editor.getContent();
        const regex = new RegExp(this.escapeRegex(this.query), 'gi');
        const newContent = content.replace(regex, this.replaceText);
        
        editor.setContent(newContent);
        
        this.findAll();
        
        if (currentDocument) {
            currentDocument.isDirty = true;
            updateTitle();
        }
    }
    
    hide() {
        this.clearHighlights();
        if (this.dialog) {
            this.dialog.remove();
            this.dialog = null;
        }
    }
}

const searchManager = new SearchManager();