# TermiEmu

A modern web-based terminal emulator with sleek UI elements inspired by Warp Terminal and similar functionality to Ghostty. Built with TypeScript, Express, and xterm.js.

## Features

🚀 **Modern UI**
- Clean, modern interface with tabbed terminals
- Dark theme with syntax highlighting
- Responsive design for desktop and mobile

⚡ **Performance**
- Built on xterm.js for fast terminal rendering
- WebSocket-based real-time communication
- Efficient PTY (pseudo-terminal) backend

🔧 **Functionality**
- Multiple terminal tabs
- Search functionality
- Web links detection and clicking
- Customizable themes and fonts
- Cross-platform support (Linux, macOS, Windows)

## Quick Start

### Prerequisites
- Node.js 18+ 
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone https://github.com/merenut/TermiEmu.git
cd TermiEmu
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

4. Open your browser and navigate to `http://localhost:3000`

### Production Build

```bash
npm run build
npm start
```

## Architecture

- **Frontend**: TypeScript + Vite + xterm.js
- **Backend**: Node.js + Express + node-pty + WebSocket
- **Communication**: WebSocket for real-time terminal I/O

## Development

### Project Structure
```
src/
├── client/          # Frontend code
│   ├── index.html   # Main HTML file
│   ├── main.ts      # Client-side TypeScript
│   └── style.css    # Styling
└── server/          # Backend code
    └── index.ts     # Server-side TypeScript
```

### Available Scripts

- `npm run dev` - Start development servers (client & server)
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm start` - Start production server

## Configuration

The terminal can be customized by modifying the terminal options in `src/client/main.ts`:

```typescript
const terminal = new Terminal({
  theme: { /* custom theme */ },
  fontFamily: 'your-preferred-font',
  fontSize: 14,
  // ... other options
});
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details.