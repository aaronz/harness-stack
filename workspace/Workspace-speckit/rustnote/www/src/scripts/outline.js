const outlineManager = {
    panel: null,
    list: null,
    headings: [],
    isVisible: false,
    
    init() {
        this.panel = document.getElementById('outline-panel');
        this.list = document.getElementById('outline-list');
        
        if (!this.panel || !this.list) {
            console.error('Outline panel elements not found');
            return;
        }
        
        this.setupEventListeners();
    },
    
    setupEventListeners() {
        document.getElementById('btn-outline')?.addEventListener('click', () => {
            this.toggle();
        });
    },
    
    toggle() {
        this.isVisible = !this.isVisible;
        this.panel.classList.toggle('hidden', !this.isVisible);
        document.getElementById('btn-outline')?.classList.toggle('active', this.isVisible);
        
        if (this.isVisible && this.headings.length > 0) {
            this.scrollToActiveHeading();
        }
    },
    
    update(headings) {
        this.headings = headings || [];
        this.render();
    },
    
    render() {
        if (!this.list) return;
        
        this.list.innerHTML = '';
        
        this.headings.forEach((heading, index) => {
            const item = document.createElement('div');
            item.className = `outline-item level-${heading.level}`;
            item.textContent = heading.text;
            item.dataset.offset = heading.offset;
            item.dataset.index = index;
            
            item.addEventListener('click', () => {
                this.navigateToHeading(heading);
            });
            
            this.list.appendChild(item);
        });
    },
    
    navigateToHeading(heading) {
        if (typeof editor.scrollToOffset === 'function') {
            editor.scrollToOffset(heading.offset);
        }
        
        this.setActiveHeading(heading);
        
        if (this.isVisible) {
            this.scrollToActiveHeading();
        }
    },
    
    setActiveHeading(heading) {
        const items = this.list?.querySelectorAll('.outline-item');
        items?.forEach(item => {
            item.classList.toggle('active', item.dataset.offset === String(heading.offset));
        });
    },
    
    scrollToActiveHeading() {
        const activeItem = this.list?.querySelector('.outline-item.active');
        if (activeItem) {
            activeItem.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    },
    
    show() {
        this.isVisible = true;
        this.panel.classList.remove('hidden');
        document.getElementById('btn-outline')?.classList.add('active');
    },
    
    hide() {
        this.isVisible = false;
        this.panel.classList.add('hidden');
        document.getElementById('btn-outline')?.classList.remove('active');
    }
};

function updateOutline(headings) {
    outlineManager.update(headings);
}

document.addEventListener('DOMContentLoaded', () => {
    outlineManager.init();
});