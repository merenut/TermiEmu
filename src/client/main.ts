import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { SearchAddon } from '@xterm/addon-search';
import '@xterm/xterm/css/xterm.css';

interface TerminalTab {
  id: string;
  terminal: Terminal;
  element: HTMLElement;
  socket: WebSocket | null;
}

class TermiEmu {
  private tabs: Map<string, TerminalTab> = new Map();
  private activeTabId: string | null = null;
  private tabCounter = 0;

  constructor() {
    this.init();
  }

  private init() {
    this.setupEventListeners();
    this.createNewTab();
  }

  private setupEventListeners() {
    // New tab button
    const newTabBtn = document.querySelector('.new-tab-btn') as HTMLButtonElement;
    newTabBtn?.addEventListener('click', () => this.createNewTab());

    // Window resize
    window.addEventListener('resize', () => this.resizeActiveTerminal());

    // Search functionality
    const searchBtn = document.querySelector('.search-btn') as HTMLButtonElement;
    searchBtn?.addEventListener('click', () => this.toggleSearch());
  }

  private createNewTab() {
    const tabId = `tab-${++this.tabCounter}`;
    
    // Create tab element
    const tabElement = this.createTabElement(tabId, `Terminal ${this.tabCounter}`);
    
    // Create terminal instance
    const terminal = new Terminal({
      theme: {
        background: '#1a1b26',
        foreground: '#c0caf5',
        cursor: '#7aa2f7',
        black: '#15161e',
        red: '#f7768e',
        green: '#9ece6a',
        yellow: '#e0af68',
        blue: '#7aa2f7',
        magenta: '#bb9af7',
        cyan: '#7dcfff',
        white: '#a9b1d6',
        brightBlack: '#414868',
        brightRed: '#f7768e',
        brightGreen: '#9ece6a',
        brightYellow: '#e0af68',
        brightBlue: '#7aa2f7',
        brightMagenta: '#bb9af7',
        brightCyan: '#7dcfff',
        brightWhite: '#c0caf5',
      },
      fontFamily: 'Menlo, Monaco, Cascadia Code, Segoe UI Mono, Roboto Mono, Oxygen Mono, Ubuntu Monospace, monospace',
      fontSize: 14,
      lineHeight: 1.4,
      cursorBlink: true,
      cursorStyle: 'block',
      allowTransparency: true,
      convertEol: true,
    });

    // Add addons
    const fitAddon = new FitAddon();
    const webLinksAddon = new WebLinksAddon();
    const searchAddon = new SearchAddon();

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);
    terminal.loadAddon(searchAddon);

    // Create terminal container
    const terminalElement = this.createTerminalElement(tabId);
    
    // Open terminal
    terminal.open(terminalElement);
    fitAddon.fit();

    // Connect to WebSocket
    const socket = this.connectWebSocket(terminal);

    // Store tab
    const tab: TerminalTab = {
      id: tabId,
      terminal,
      element: terminalElement,
      socket,
    };

    this.tabs.set(tabId, tab);
    this.switchToTab(tabId);

    return tab;
  }

  private createTabElement(tabId: string, title: string): HTMLElement {
    const tabsContainer = document.querySelector('.terminal-tabs') as HTMLElement;
    const newTabBtn = tabsContainer.querySelector('.new-tab-btn') as HTMLElement;

    const tabElement = document.createElement('div');
    tabElement.className = 'tab';
    tabElement.dataset.tab = tabId;
    
    tabElement.innerHTML = `
      <span class="tab-title">${title}</span>
      <button class="tab-close">×</button>
    `;

    // Add event listeners
    tabElement.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      if (!target.classList.contains('tab-close')) {
        this.switchToTab(tabId);
      }
    });

    const closeBtn = tabElement.querySelector('.tab-close') as HTMLButtonElement;
    closeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      this.closeTab(tabId);
    });

    tabsContainer.insertBefore(tabElement, newTabBtn);
    return tabElement;
  }

  private createTerminalElement(tabId: string): HTMLElement {
    const content = document.querySelector('.terminal-content') as HTMLElement;
    
    const terminalElement = document.createElement('div');
    terminalElement.id = `terminal-${tabId}`;
    terminalElement.className = 'terminal-instance';
    
    content.appendChild(terminalElement);
    return terminalElement;
  }

  private connectWebSocket(terminal: Terminal): WebSocket | null {
    try {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const host = window.location.hostname;
      const port = '3001'; // Server port
      const socket = new WebSocket(`${protocol}//${host}:${port}/ws`);

      socket.onopen = () => {
        console.log('WebSocket connected');
        terminal.write('\\r\\n*** Connected to TermiEmu ***\\r\\n\\r\\n');
      };

      socket.onmessage = (event) => {
        terminal.write(event.data);
      };

      socket.onclose = () => {
        console.log('WebSocket disconnected');
        terminal.write('\\r\\n*** Connection lost ***\\r\\n');
      };

      socket.onerror = (error) => {
        console.error('WebSocket error:', error);
        terminal.write('\\r\\n*** Connection error ***\\r\\n');
      };

      // Send terminal input to server
      terminal.onData((data) => {
        if (socket.readyState === WebSocket.OPEN) {
          socket.send(data);
        }
      });

      return socket;
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      terminal.write('\\r\\n*** Failed to connect to server ***\\r\\n');
      return null;
    }
  }

  private switchToTab(tabId: string) {
    // Update active tab in UI
    document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
    document.querySelectorAll('.terminal-instance').forEach(term => term.classList.remove('active'));

    const tabElement = document.querySelector(`[data-tab="${tabId}"]`) as HTMLElement;
    const terminalElement = document.querySelector(`#terminal-${tabId}`) as HTMLElement;

    if (tabElement && terminalElement) {
      tabElement.classList.add('active');
      terminalElement.classList.add('active');
      this.activeTabId = tabId;

      // Fit terminal when switching
      setTimeout(() => this.resizeActiveTerminal(), 100);
    }
  }

  private closeTab(tabId: string) {
    const tab = this.tabs.get(tabId);
    if (!tab) return;

    // Close WebSocket
    if (tab.socket) {
      tab.socket.close();
    }

    // Dispose terminal
    tab.terminal.dispose();

    // Remove DOM elements
    const tabElement = document.querySelector(`[data-tab="${tabId}"]`);
    const terminalElement = document.querySelector(`#terminal-${tabId}`);
    
    tabElement?.remove();
    terminalElement?.remove();

    // Remove from tabs map
    this.tabs.delete(tabId);

    // If this was the active tab, switch to another
    if (this.activeTabId === tabId) {
      const remainingTabs = Array.from(this.tabs.keys());
      if (remainingTabs.length > 0) {
        this.switchToTab(remainingTabs[remainingTabs.length - 1]);
      } else {
        // Create a new tab if no tabs remain
        this.createNewTab();
      }
    }
  }

  private resizeActiveTerminal() {
    if (this.activeTabId) {
      const tab = this.tabs.get(this.activeTabId);
      if (tab && tab.terminal) {
        // Get the fit addon from the terminal
        const fitAddon = (tab.terminal as any)._addonManager?._addons?.find(
          (addon: any) => addon.instance instanceof FitAddon
        )?.instance as FitAddon;
        
        if (fitAddon) {
          fitAddon.fit();
        }
      }
    }
  }

  private toggleSearch() {
    if (this.activeTabId) {
      const tab = this.tabs.get(this.activeTabId);
      if (tab && tab.terminal) {
        // This would typically open a search dialog
        // For now, we'll just focus the terminal
        tab.terminal.focus();
        console.log('Search functionality - to be implemented');
      }
    }
  }
}

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
  new TermiEmu();
});

// Handle page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    new TermiEmu();
  });
} else {
  new TermiEmu();
}