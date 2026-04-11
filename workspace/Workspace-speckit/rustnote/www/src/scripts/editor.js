/**
 * RustNote WYSIWYM Editor
 * 
 * Single contenteditable editor with inline markdown rendering.
 * Integrates with Rust backend via Tauri IPC.
 */

const wysiwygEditor = {
    container: null,
    sourceTextarea: null,
    
    content: '',
    semanticDoc: null,
    cursorOffset: 0,
    selectionRange: null,
    
    focusMode: false,
    typewriterMode: false,
    activeParagraph: null,
    
    isRendering: false,
    renderDebounce: null,
    highlightCode: true,
    
    decorations: [],
    cursorMapping: [],
    
    init() {
        this.container = document.getElementById('editor-content');
        this.sourceTextarea = document.getElementById('source');
        
        if (!this.container) {
            console.error('Editor container not found');
            return;
        }
        
        this.setupEventListeners();
        this.render();
    },
    
    setupEventListeners() {
        this.container.addEventListener('input', (e) => {
            if (currentDocument) {
                currentDocument.isDirty = true;
                updateTitle();
            }
            this.handleInput();
        });
        
        this.container.addEventListener('keydown', (e) => {
            this.handleKeydown(e);
        });
        
        this.container.addEventListener('paste', async (e) => {
            const handledImage = await this.handleImagePaste(e);
            if (!handledImage) {
                e.preventDefault();
                const text = e.clipboardData.getData('text/plain');
                this.insertText(text);
            }
        });
        
        this.container.addEventListener('dragover', (e) => {
            this.handleImageDragOver(e);
        });
        
        this.container.addEventListener('drop', (e) => {
            this.handleImageDrop(e);
        });
        
        this.container.addEventListener('click', (e) => {
            this.updateCursorOffset();
            this.handleLinkClick(e);
        });
        
        this.container.addEventListener('select', () => {
            this.updateSelection();
        });
        
        document.addEventListener('selectionchange', () => {
            this.handleSelectionChange();
        });
    },
    
    handleInput() {
        this.content = this.getPlainText();
        this.updateCursorOffset();
        this.debouncedRender(50);
        this.syncToBackend();
    },
    
    handleKeydown(e) {
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const modifier = isMac ? e.metaKey : e.ctrlKey;
        
        if (this.handleTableNavKey(e)) {
            return;
        }
        
        if (modifier) {
            switch(e.key.toLowerCase()) {
                case 'b':
                    e.preventDefault();
                    this.formatBold();
                    break;
                case 'i':
                    e.preventDefault();
                    this.formatItalic();
                    break;
                case 's':
                    e.preventDefault();
                    if (typeof saveDocument === 'function') saveDocument();
                    break;
                case 'z':
                    e.preventDefault();
                    if (e.shiftKey) {
                        this.redo();
                    } else {
                        this.undo();
                    }
                    break;
                case 'y':
                    e.preventDefault();
                    this.redo();
                    break;
            }
            return;
        }
        
        if (e.key === 'Enter') {
            this.handleEnter(e);
        } else if (e.key === 'Backspace') {
            this.handleBackspace(e);
        } else if (e.key === 'Tab') {
            e.preventDefault();
            this.handleTab(e.shiftKey);
        }
    },
    
    handleSelectionChange() {
        const selection = window.getSelection();
        if (selection.rangeCount > 0) {
            const range = selection.getRangeAt(0);
            this.selectionRange = range;
            this.updateCursorFromDOM(range);
        }
    },
    
    updateCursorFromDOM(range) {
        if (!this.container.contains(range.startContainer)) {
            return;
        }
        
        const preCaretRange = range.cloneRange();
        preCaretRange.selectNodeContents(this.container);
        preCaretRange.setEnd(range.startContainer, range.startOffset);
        const start = preCaretRange.toString().length;
        
        this.cursorOffset = Math.min(start, this.content.length);
        
        if (this.focusMode) {
            this.updateActiveParagraph();
        }
        
        if (this.typewriterMode) {
            this.scrollToCenter();
        }
    },
    
    async handleEnter(e) {
        const start = this.cursorOffset;
        const content = this.content;
        
        try {
            const result = await window.__TAURI__.core.invoke('editor_apply_transform', {
                transform: { Enter: null },
                content: content,
                cursorOffset: start
            });
            
            if (result && result.content !== content) {
                e.preventDefault();
                this.setContent(result.content);
                this.setCursorPosition(result.cursor_offset);
                return;
            }
        } catch (err) {
            console.error('Transform error:', err);
        }
        
        this.insertText('\n');
    },
    
    async handleBackspace(e) {
        const start = this.cursorOffset;
        const content = this.content;
        
        if (start === 0) {
            return;
        }
        
        try {
            const result = await window.__TAURI__.core.invoke('editor_apply_transform', {
                transform: { Backspace: null },
                content: content,
                cursorOffset: start
            });
            
            if (result && result.content !== content) {
                e.preventDefault();
                this.setContent(result.content);
                this.setCursorPosition(result.cursor_offset);
                return;
            }
        } catch (err) {
            console.error('Backspace transform error:', err);
        }
    },
    
    async handleTab(shiftKey) {
        const start = this.cursorOffset;
        const content = this.content;
        
        try {
            const result = await window.__TAURI__.core.invoke('editor_apply_transform', {
                transform: shiftKey ? { ShiftTab: null } : { Tab: null },
                content: content,
                cursorOffset: start
            });
            
            if (result && result.content !== content) {
                e.preventDefault();
                this.setContent(result.content);
                this.setCursorPosition(result.cursor_offset);
                return;
            }
        } catch (err) {
            console.error('Tab transform error:', err);
        }
        
        this.insertText('    ');
    },
    
    async handleLinkClick(e) {
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const modifier = isMac ? e.metaKey : e.ctrlKey;
        
        if (!modifier) return;
        
        const link = e.target.closest('a');
        if (!link) return;
        
        e.preventDefault();
        const href = link.getAttribute('href');
        
        if (!href) return;
        
        if (href.startsWith('http://') || href.startsWith('https://')) {
            try {
                await window.__TAURI__.core.invoke('open_external_url', { url: href });
            } catch (err) {
                console.error('Failed to open URL:', err);
            }
        } else if (href.startsWith('/') || href.startsWith('./') || href.startsWith('../')) {
            try {
                await window.__TAURI__.core.invoke('open_external_url', { url: href });
            } catch (err) {
                console.error('Failed to open relative URL:', err);
            }
        }
    },
    
    isInTableCell() {
        return this.container && this.container.contains(window.getSelection()?.anchorNode);
    },
    
    getTableCellAtCursor() {
        const selection = window.getSelection();
        if (!selection || !selection.rangeCount) return null;
        
        const range = selection.getRangeAt(0);
        const node = range.startContainer;
        
        let cell = node;
        if (cell.nodeType === Node.TEXT_NODE) {
            cell = cell.parentNode;
        }
        
        while (cell && cell.tagName !== 'TD' && cell.tagName !== 'TH') {
            cell = cell.parentNode;
            if (!cell || cell === this.container) return null;
        }
        
        return cell;
    },
    
    getNextTableCell(currentCell, shiftKey) {
        if (!currentCell) return null;
        
        const table = currentCell.closest('table');
        if (!table) return null;
        
        const cells = Array.from(table.querySelectorAll('td, th'));
        const currentIndex = cells.indexOf(currentCell);
        
        if (currentIndex === -1) return null;
        
        if (shiftKey) {
            return currentIndex > 0 ? cells[currentIndex - 1] : null;
        } else {
            return currentIndex < cells.length - 1 ? cells[currentIndex + 1] : null;
        }
    },
    
    navigateToTableCell(cell) {
        if (!cell) return;
        
        cell.focus();
        
        const range = document.createRange();
        range.selectNodeContents(cell);
        range.collapse(false);
        
        const selection = window.getSelection();
        selection.removeAllRanges();
        selection.addRange(range);
        
        this.updateCursorOffset();
    },
    
    handleTableNavKey(e) {
        if (!this.isInTableCell()) return false;
        
        if (e.key === 'Tab') {
            e.preventDefault();
            const currentCell = this.getTableCellAtCursor();
            const nextCell = this.getNextTableCell(currentCell, e.shiftKey);
            this.navigateToTableCell(nextCell);
            return true;
        }
        
        if (e.key === 'Enter' && !e.shiftKey) {
            const currentCell = this.getTableCellAtCursor();
            if (currentCell) {
                e.preventDefault();
                
                const selection = window.getSelection();
                if (!selection.rangeCount) return true;
                
                const range = selection.getRangeAt(0);
                
                if (range.startContainer.nodeType === Node.TEXT_NODE && 
                    range.startOffset < range.startContainer.length) {
                    range.splitText(range.startOffset);
                }
                
                const textNode = document.createTextNode('\n');
                range.insertNode(textNode);
                
                range.setStartAfter(textNode);
                range.collapse(true);
                selection.removeAllRanges();
                selection.addRange(range);
                
                this.content = this.getPlainText();
                return true;
            }
        }
        
        return false;
    },
    
    async syncToBackend() {
        if (!currentDocument) return;
        
        try {
            const result = await window.__TAURI__.core.invoke('render_for_editor', {
                markdown: this.content,
                cursorOffset: this.cursorOffset
            });
            
            if (result) {
                this.semanticDoc = result;
                this.cursorMapping = result.cursor_mapping || [];
                this.activeParagraph = result.active_paragraph;
                
                if (typeof updateOutline === 'function') {
                    updateOutline(result.headings || []);
                }
            }
        } catch (err) {
            console.error('Sync error:', err);
        }
    },
    
    debouncedRender(delay = 100) {
        if (this.renderDebounce) {
            clearTimeout(this.renderDebounce);
        }
        this.renderDebounce = setTimeout(() => {
            this.render();
        }, delay);
    },
    
    async render() {
        if (this.isRendering) return;
        this.isRendering = true;
        
        try {
            const result = await window.__TAURI__.core.invoke('render_for_editor_with_highlighting', {
                markdown: this.content,
                cursorOffset: this.cursorOffset,
                includeHighlighting: this.highlightCode
            });
            
            if (result) {
                this.semanticDoc = result;
                this.cursorMapping = result.cursor_mapping || [];
                this.activeParagraph = result.active_paragraph;
                
                this.applyDecorations(result.html);
            }
        } catch (e) {
            console.error('Render error:', e);
        } finally {
            this.isRendering = false;
        }
    },
    
    applyDecorations(html) {
        const cursorPosition = this.getDOMCursorPosition();
        
        const wrappedHtml = this.wrapWithDecorations(html);
        
        this.container.innerHTML = wrappedHtml;
        
        this.restoreCursorPosition(cursorPosition);
        
        this.updateActiveParagraphClass();
    },
    
    wrapWithDecorations(html) {
        let decorated = html;
        
        decorated = decorated.replace(/\*\*(.+?)\*\*/g, '<span class="md-bold" data-md="**">**$1**</span>');
        decorated = decorated.replace(/__(.+?)__/g, '<span class="md-bold" data-md="__">__$1__</span>');
        
        decorated = decorated.replace(/\*(.+?)\*/g, '<span class="md-italic" data-md="*">*$1*</span>');
        decorated = decorated.replace(/_(.+?)_/g, '<span class="md-italic" data-md="_">_$1_</span>');
        
        decorated = decorated.replace(/`(.+?)`/g, '<span class="md-code" data-md="`">`$1`</span>');
        
        decorated = decorated.replace(/^### (.+)$/gm, '<span class="md-heading md-heading-3" data-md="### ">### $1</span>');
        decorated = decorated.replace(/^## (.+)$/gm, '<span class="md-heading md-heading-2" data-md="## ">## $1</span>');
        decorated = decorated.replace(/^# (.+)$/gm, '<span class="md-heading md-heading-1" data-md="# "># $1</span>');
        
        decorated = decorated.replace(/^> (.+)$/gm, '<span class="md-blockquote" data-md="> ">> $1</span>');
        
        decorated = decorated.replace(/^- \[ \] (.+)$/gm, '<span class="md-task-list-item md-task-unchecked" data-md="- [ ] ">- [ ] $1</span>');
        decorated = decorated.replace(/^- \[x\] (.+)$/gi, '<span class="md-task-list-item md-task-checked" data-md="- [x] ">- [x] $1</span>');
        decorated = decorated.replace(/^- (.+)$/gm, '<span class="md-list-item" data-md="- ">- $1</span>');
        
        return decorated;
    },
    
    getDOMCursorPosition() {
        const selection = window.getSelection();
        if (selection.rangeCount === 0) return 0;
        
        const range = selection.getRangeAt(0);
        const preCaretRange = range.cloneRange();
        preCaretRange.selectNodeContents(this.container);
        preCaretRange.setEnd(range.startContainer, range.startOffset);
        return preCaretRange.toString().length;
    },
    
    restoreCursorPosition(domPosition) {
        const range = document.createRange();
        const selection = window.getSelection();
        
        let charCount = 0;
        let found = false;
        
        function traverseNodes(node) {
            if (found) return;
            
            if (node.nodeType === Node.TEXT_NODE) {
                const nextCount = charCount + node.length;
                if (domPosition <= nextCount) {
                    range.setStart(node, domPosition - charCount);
                    range.collapse(true);
                    found = true;
                }
                charCount = nextCount;
            } else {
                for (let child of node.childNodes) {
                    traverseNodes(child);
                    if (found) return;
                }
            }
        }
        
        traverseNodes(this.container);
        
        if (!found) {
            range.selectNodeContents(this.container);
            range.collapse(false);
        }
        
        selection.removeAllRanges();
        selection.addRange(range);
    },
    
    updateCursorOffset() {
        const position = this.getDOMCursorPosition();
        this.cursorOffset = position;
    },
    
    setCursorPosition(offset) {
        this.cursorOffset = Math.min(offset, this.content.length);
        this.restoreCursorPosition(this.cursorOffset);
    },
    
    getPlainText() {
        return this.container.textContent || '';
    },
    
    getContent() {
        return this.content;
    },
    
    setContent(text) {
        this.content = text;
        this.render();
    },
    
    insertText(text) {
        const selection = window.getSelection();
        if (!selection.rangeCount) return;
        
        const range = selection.getRangeAt(0);
        range.deleteContents();
        
        const textNode = document.createTextNode(text);
        range.insertNode(textNode);
        
        range.setStartAfter(textNode);
        range.collapse(true);
        selection.removeAllRanges();
        selection.addRange(range);
        
        this.content = this.getPlainText();
        this.cursorOffset += text.length;
        this.debouncedRender(50);
        this.syncToBackend();
    },
    
    formatBold() {
        this.wrapSelection('**', '**');
    },
    
    formatItalic() {
        this.wrapSelection('*', '*');
    },
    
    formatCode() {
        this.wrapSelection('`', '`');
    },
    
    wrapSelection(before, after) {
        const selection = window.getSelection();
        if (!selection.rangeCount) return;
        
        const range = selection.getRangeAt(0);
        const selectedText = range.toString();
        
        if (!selectedText) return;
        
        const wrapper = document.createElement('span');
        wrapper.className = `md-${before === '**' ? 'bold' : before === '*' ? 'italic' : 'code'}`;
        wrapper.setAttribute('data-md', before);
        wrapper.textContent = selectedText;
        
        range.deleteContents();
        range.insertNode(wrapper);
        
        range.setStartAfter(wrapper);
        range.collapse(true);
        selection.removeAllRanges();
        selection.addRange(range);
        
        this.content = this.getPlainText();
        this.debouncedRender(50);
        this.syncToBackend();
    },
    
    toggleFocusMode() {
        this.focusMode = !this.focusMode;
        this.container.classList.toggle('focus-mode', this.focusMode);
        
        if (this.focusMode) {
            this.updateActiveParagraph();
        }
        
        return this.focusMode;
    },
    
    toggleTypewriterMode() {
        this.typewriterMode = !this.typewriterMode;
        this.container.classList.toggle('typewriter-mode', this.typewriterMode);
        
        if (this.typewriterMode) {
            this.scrollToCenter();
        }
        
        return this.typewriterMode;
    },
    
    updateActiveParagraph() {
        if (!this.semanticDoc || this.semanticDoc.active_paragraph === undefined) {
            this.activeParagraph = 0;
        } else {
            this.activeParagraph = this.semanticDoc.active_paragraph;
        }
        
        this.updateActiveParagraphClass();
    },
    
    updateActiveParagraphClass() {
        if (!this.focusMode) return;
        
        const paragraphs = this.container.querySelectorAll('.paragraph, p, .md-heading');
        paragraphs.forEach((p, i) => {
            p.classList.toggle('active', i === this.activeParagraph);
        });
    },
    
    scrollToCenter() {
        const selection = window.getSelection();
        if (!selection.rangeCount) return;
        
        const range = selection.getRangeAt(0);
        const rect = range.getBoundingClientRect();
        const containerRect = this.container.getBoundingClientRect();
        
        const targetTop = containerRect.top + (containerRect.height / 2) - (rect.height / 2);
        const currentTop = this.container.scrollTop;
        
        this.container.scrollTop = currentTop + (targetTop - containerRect.top - containerRect.height / 2 + rect.height / 2);
    },
    
    scrollToOffset(offset) {
        if (!this.container) return;
        
        const mappings = this.cursorMapping || [];
        let domPosition = offset;
        
        if (mappings.length > 0) {
            for (const mapping of mappings) {
                if (mapping.source_offset <= offset) {
                    const delta = offset - mapping.source_offset;
                    domPosition = mapping.dom_offset + delta;
                } else {
                    break;
                }
            }
        }
        
        const range = document.createRange();
        const selection = window.getSelection();
        
        let charCount = 0;
        let found = false;
        
        const traverseNodes = (node) => {
            if (found) return;
            
            if (node.nodeType === Node.TEXT_NODE) {
                const nextCount = charCount + node.length;
                if (domPosition <= nextCount) {
                    range.setStart(node, domPosition - charCount);
                    range.collapse(true);
                    found = true;
                }
                charCount = nextCount;
            } else {
                for (let child of node.childNodes) {
                    traverseNodes(child);
                    if (found) return;
                }
            }
        };
        
        traverseNodes(this.container);
        
        if (!found) {
            const ratio = offset / Math.max(1, this.content.length);
            this.container.scrollTop = ratio * this.container.scrollHeight;
            return;
        }
        
        const rect = range.getBoundingClientRect();
        const containerRect = this.container.getBoundingClientRect();
        
        this.container.scrollTop = this.container.scrollTop + rect.top - containerRect.top - containerRect.height / 2 + rect.height / 2;
        
        selection.removeAllRanges();
        selection.addRange(range);
    },
    
    undo() {
        document.execCommand('undo');
        this.debouncedRender();
    },
    
    redo() {
        document.execCommand('redo');
        this.debouncedRender();
    },
    
    updateSelection() {
        this.updateCursorOffset();
    },
    
    async getMarkdownInfo() {
        try {
            return await window.__TAURI__.core.invoke('get_markdown_info', { 
                markdown: this.content 
            });
        } catch (e) {
            console.error('Error getting markdown info:', e);
            return null;
        }
    },
    
    async insertImage() {
        try {
            const result = await window.__TAURI__.dialog.open({
                filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'] }],
                multiple: false
            });
            
            if (result) {
                const workspacePath = currentDocument?.filePath 
                    ? currentDocument.filePath.substring(0, currentDocument.filePath.lastIndexOf('/'))
                    : null;
                
                const imageInfo = await window.__TAURI__.core.invoke('insert_image', {
                    imagePath: result,
                    workspacePath: workspacePath
                });
                
                if (imageInfo) {
                    const markdown = `![${imageInfo.file_name}](${imageInfo.relative_path})`;
                    this.insertText(markdown);
                    return true;
                }
            }
        } catch (e) {
            console.error('Error inserting image:', e);
        }
        return false;
    },
    
    async handleImageDrop(e) {
        e.preventDefault();
        e.stopPropagation();
        
        const files = e.dataTransfer?.files;
        if (!files || files.length === 0) return;
        
        const imageFile = Array.from(files).find(f => 
            f.type.startsWith('image/')
        );
        
        if (!imageFile) return;
        
        try {
            const arrayBuffer = await imageFile.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);
            const base64 = btoa(String.fromCharCode.apply(null, uint8Array));
            const mimeType = imageFile.type;
            const dataUrl = `data:${mimeType};base64,${base64}`;
            
            const workspacePath = currentDocument?.filePath 
                ? currentDocument.filePath.substring(0, currentDocument.filePath.lastIndexOf('/'))
                : null;
            
            const tempPath = `/tmp/${imageFile.name}`;
            
            this.insertText(`![${imageFile.name}](${dataUrl})`);
            
            if (currentDocument) {
                currentDocument.isDirty = true;
                updateTitle();
            }
        } catch (err) {
            console.error('Error handling image drop:', err);
        }
    },
    
    handleImageDragOver(e) {
        e.preventDefault();
        e.stopPropagation();
    },
    
    async handleImagePaste(e) {
        const items = e.clipboardData?.items;
        if (!items) return;
        
        for (const item of items) {
            if (item.type.startsWith('image/')) {
                e.preventDefault();
                
                const file = item.getAsFile();
                if (!file) continue;
                
                try {
                    const arrayBuffer = await file.arrayBuffer();
                    const uint8Array = new Uint8Array(arrayBuffer);
                    const base64 = btoa(String.fromCharCode.apply(null, uint8Array));
                    const mimeType = file.type || 'image/png';
                    const dataUrl = `data:${mimeType};base64,${base64}`;
                    
                    this.insertText(`![Pasted Image](${dataUrl})`);
                    
                    if (currentDocument) {
                        currentDocument.isDirty = true;
                        updateTitle();
                    }
                    
                    return true;
                } catch (err) {
                    console.error('Error handling image paste:', err);
                }
            }
        }
        return false;
    }
};

const editor = wysiwygEditor;

document.addEventListener('DOMContentLoaded', () => {
    editor.init();
});