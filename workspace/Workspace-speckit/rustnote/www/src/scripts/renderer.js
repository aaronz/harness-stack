class MarkdownRenderer {
    constructor() {
        this.debounceTime = 150;
        this.timeout = null;
    }
    
    async render(markdown, callback) {
        clearTimeout(this.timeout);
        this.timeout = setTimeout(async () => {
            try {
                const html = await window.__TAURI__.core.invoke('render_markdown', { markdown });
                callback(html);
            } catch (e) {
                console.error('Render error:', e);
                callback(this.fallbackParse(markdown));
            }
        }, this.debounceTime);
    }
    
    fallbackParse(markdown) {
        let html = markdown
            .replace(/^###### (.+)$/gm, '<h6>$1</h6>')
            .replace(/^##### (.+)$/gm, '<h5>$1</h5>')
            .replace(/^#### (.+)$/gm, '<h4>$1</h4>')
            .replace(/^### (.+)$/gm, '<h3>$1</h3>')
            .replace(/^## (.+)$/gm, '<h2>$1</h2>')
            .replace(/^# (.+)$/gm, '<h1>$1</h1>')
            .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
            .replace(/\*(.+?)\*/g, '<em>$1</em>')
            .replace(/`(.+?)`/g, '<code>$1</code>')
            .replace(/\[(.+?)\]\((.+?)\)/g, '<a href="$2">$1</a>')
            .replace(/- \[ \] /g, '<input type="checkbox" disabled> ')
            .replace(/- \[x\] /g, '<input type="checkbox" checked disabled> ');
        
        return html;
    }
}

const renderer = new MarkdownRenderer();