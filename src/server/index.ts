import express, { Request, Response } from 'express';
import { WebSocketServer } from 'ws';
import { spawn } from 'node-pty';
import { createServer } from 'http';
import cors from 'cors';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import os from 'os';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

interface TerminalSession {
  id: string;
  ptyProcess: any;
  socket: any;
}

class TerminalServer {
  private app = express();
  private server = createServer(this.app);
  private wss = new WebSocketServer({ server: this.server });
  private sessions = new Map<string, TerminalSession>();
  private sessionCounter = 0;

  constructor() {
    this.setupExpress();
    this.setupWebSocket();
  }

  private setupExpress() {
    // Enable CORS
    this.app.use(cors());
    
    // Parse JSON
    this.app.use(express.json());

    // Serve static files from dist/client in production
    const clientPath = join(__dirname, '../client');
    this.app.use(express.static(clientPath));

    // API routes
    this.app.get('/api/health', (req: Request, res: Response) => {
      res.json({ status: 'ok', sessions: this.sessions.size });
    });

    this.app.get('/api/sessions', (req: Request, res: Response) => {
      const sessionIds = Array.from(this.sessions.keys());
      res.json({ sessions: sessionIds });
    });

    // Fallback to index.html for SPA - handle any unmatched routes
    this.app.use((req: Request, res: Response) => {
      res.sendFile(join(clientPath, 'index.html'));
    });
  }

  private setupWebSocket() {
    this.wss.on('connection', (ws, req) => {
      console.log('New WebSocket connection');
      
      const sessionId = this.createSession(ws);
      
      ws.on('message', (data) => {
        const session = this.sessions.get(sessionId);
        if (session && session.ptyProcess) {
          session.ptyProcess.write(data.toString());
        }
      });

      ws.on('close', () => {
        console.log(`WebSocket closed for session ${sessionId}`);
        this.destroySession(sessionId);
      });

      ws.on('error', (error) => {
        console.error(`WebSocket error for session ${sessionId}:`, error);
        this.destroySession(sessionId);
      });
    });
  }

  private createSession(socket: any): string {
    const sessionId = `session-${++this.sessionCounter}`;
    
    try {
      // Determine shell to use
      const shell = process.env.SHELL || (os.platform() === 'win32' ? 'powershell.exe' : '/bin/bash');
      
      // Create PTY process
      const ptyProcess = spawn(shell, [], {
        name: 'xterm-color',
        cols: 80,
        rows: 24,
        cwd: process.env.HOME || process.env.USERPROFILE || os.homedir(),
        env: {
          ...process.env,
          TERM: 'xterm-256color',
          COLORTERM: 'truecolor',
        },
      });

      // Handle PTY data
      ptyProcess.onData((data: string) => {
        if (socket.readyState === 1) { // WebSocket.OPEN
          socket.send(data);
        }
      });

      // Handle PTY exit
      ptyProcess.onExit(({ exitCode, signal }) => {
        console.log(`PTY process exited for session ${sessionId}:`, { exitCode, signal });
        socket.send(`\\r\\n*** Process exited with code ${exitCode} ***\\r\\n`);
        this.destroySession(sessionId);
      });

      const session: TerminalSession = {
        id: sessionId,
        ptyProcess,
        socket,
      };

      this.sessions.set(sessionId, session);
      console.log(`Created session ${sessionId}`);

      return sessionId;
    } catch (error) {
      console.error(`Failed to create session ${sessionId}:`, error);
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      socket.send(`\\r\\n*** Failed to create terminal session: ${errorMessage} ***\\r\\n`);
      return sessionId;
    }
  }

  private destroySession(sessionId: string) {
    const session = this.sessions.get(sessionId);
    if (session) {
      try {
        if (session.ptyProcess) {
          session.ptyProcess.kill();
        }
      } catch (error) {
        console.error(`Error killing PTY process for session ${sessionId}:`, error);
      }

      this.sessions.delete(sessionId);
      console.log(`Destroyed session ${sessionId}`);
    }
  }

  public start(port: number = 3001) {
    this.server.listen(port, () => {
      console.log(`TermiEmu server running on port ${port}`);
      console.log(`WebSocket server ready`);
      console.log(`Available shells: ${process.env.SHELL || 'default system shell'}`);
    });

    // Graceful shutdown
    process.on('SIGTERM', () => {
      console.log('Received SIGTERM, shutting down gracefully');
      this.shutdown();
    });

    process.on('SIGINT', () => {
      console.log('Received SIGINT, shutting down gracefully');
      this.shutdown();
    });
  }

  private shutdown() {
    // Close all sessions
    for (const [sessionId, session] of this.sessions) {
      this.destroySession(sessionId);
    }

    // Close WebSocket server
    this.wss.close(() => {
      console.log('WebSocket server closed');
    });

    // Close HTTP server
    this.server.close(() => {
      console.log('HTTP server closed');
      process.exit(0);
    });
  }
}

// Start the server
const server = new TerminalServer();
server.start(parseInt(process.env.PORT || '3001'));