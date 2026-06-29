import React, { useState, useRef, useEffect } from 'react';
import { Send, Sparkles, Bot, User, CornerDownLeft, Loader2, RefreshCw } from 'lucide-react';

interface Message {
  id: string;
  role: 'user' | 'model';
  text: string;
  timestamp: Date;
}

export function ChatAssistant() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 'welcome',
      role: 'model',
      text: 'Hello! I am your AI Design Assistant. Ask me to draft SKiDL code, optimize layer stackups, analyze connectivity, or review DRC constraints.',
      timestamp: new Date()
    }
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = async (textToSend: string) => {
    if (!textToSend.trim() || isLoading) return;

    const userMsg: Message = {
      id: crypto.randomUUID(),
      role: 'user',
      text: textToSend,
      timestamp: new Date()
    };

    setMessages((prev) => [...prev, userMsg]);
    setInput('');
    setIsLoading(true);

    try {
      // Build simplified history
      const historyPayload = messages.map(m => ({
        role: m.role,
        text: m.text
      }));

      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: textToSend,
          history: historyPayload
        })
      });

      if (!response.ok) {
        throw new Error('Assistant offline');
      }

      const data = await response.json();
      const modelMsg: Message = {
        id: crypto.randomUUID(),
        role: 'model',
        text: data.text || 'Sorry, I couldn\'t find a solution for that query.',
        timestamp: new Date()
      };
      setMessages((prev) => [...prev, modelMsg]);
    } catch (err: any) {
      const errorMsg: Message = {
        id: crypto.randomUUID(),
        role: 'model',
        text: `Error connecting to the local design agent: ${err.message || 'Please verify GEMINI_API_KEY is configured.'}`,
        timestamp: new Date()
      };
      setMessages((prev) => [...prev, errorMsg]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend(input);
    }
  };

  const suggestionChips = [
    "Write SKiDL bypass circuit",
    "Design 4-layer stackup",
    "Identify unrouted traces",
    "Check power trace width"
  ];

  return (
    <div className="flex flex-col h-full bg-[#141417]/80 backdrop-blur rounded-xl border border-white/5 overflow-hidden shadow-2xl relative">
      {/* Header */}
      <div className="h-12 border-b border-white/5 flex items-center justify-between px-4 bg-[#0F0F11]">
        <div className="flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-teal-400" />
          <span className="text-xs uppercase tracking-widest text-slate-200 font-bold">Lattice Cohort AI</span>
        </div>
        {messages.length > 1 && (
          <button 
            onClick={() => setMessages([messages[0]])}
            className="text-slate-500 hover:text-white transition-colors p-1 rounded hover:bg-white/5"
            title="Reset Conversation"
          >
            <RefreshCw className="w-3.3 h-3.3" />
          </button>
        )}
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 min-h-0 text-xs">
        {messages.map((msg) => (
          <div 
            key={msg.id} 
            className={`flex gap-3 max-w-[88%] ${msg.role === 'user' ? 'ml-auto flex-row-reverse' : 'mr-auto'}`}
          >
            <div className={`w-6 h-6 rounded flex items-center justify-center shrink-0 ${
              msg.role === 'user' 
                ? 'bg-teal-500/10 border border-teal-500/20 text-teal-400' 
                : 'bg-white/5 border border-white/10 text-slate-300'
            }`}>
              {msg.role === 'user' ? <User className="w-3.5 h-3.5" /> : <Bot className="w-3.5 h-3.5" />}
            </div>
            
            <div className={`p-3 rounded-lg leading-relaxed whitespace-pre-wrap ${
              msg.role === 'user'
                ? 'bg-teal-600/10 border border-teal-500/10 text-slate-200'
                : 'bg-white/5 border border-white/5 text-slate-300'
            }`}>
              {msg.text}
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex gap-3 mr-auto max-w-[85%]">
            <div className="w-6 h-6 rounded flex items-center justify-center shrink-0 bg-white/5 border border-white/10 text-slate-300">
              <Bot className="w-3.5 h-3.5 animate-pulse text-teal-400" />
            </div>
            <div className="p-3 rounded-lg bg-white/5 border border-white/5 text-slate-500 italic flex items-center gap-2">
              <Loader2 className="w-3 h-3 animate-spin text-teal-400" />
              Synthesizing network parameters...
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Suggestion Chips */}
      {messages.length === 1 && !isLoading && (
        <div className="px-4 pb-2">
          <div className="text-[10px] uppercase tracking-widest text-slate-500 mb-2 font-semibold">Suggested Tasks</div>
          <div className="grid grid-cols-2 gap-2">
            {suggestionChips.map((chip, idx) => (
              <button
                key={idx}
                onClick={() => handleSend(chip)}
                className="p-2 text-left bg-white/5 border border-white/5 hover:border-teal-500/30 hover:bg-white/10 transition-all rounded text-[11px] text-slate-300 hover:text-white truncate"
              >
                {chip}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Input */}
      <div className="p-3 border-t border-white/5 bg-[#0F0F11]">
        <div className="relative flex items-center bg-white/5 border border-white/10 focus-within:border-teal-500/50 rounded-lg pr-2 py-1 transition-all">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyPress}
            placeholder="Ask agent / compile instructions..."
            rows={2}
            className="w-full bg-transparent border-0 outline-none focus:ring-0 text-slate-200 text-xs px-3 py-1.5 resize-none placeholder-slate-500"
          />
          <button
            onClick={() => handleSend(input)}
            disabled={!input.trim() || isLoading}
            className={`w-8 h-8 rounded-md flex items-center justify-center transition-all shrink-0 ${
              input.trim() && !isLoading
                ? 'bg-teal-600 hover:bg-teal-500 text-white shadow-lg shadow-teal-500/15 cursor-pointer'
                : 'bg-white/5 text-slate-600 cursor-not-allowed'
            }`}
          >
            <Send className="w-3.5 h-3.5" />
          </button>
        </div>
        <div className="flex justify-between items-center mt-1.5 px-1 text-[9px] text-slate-500 font-mono">
          <span>GEMINI-3.5-FLASH</span>
          <span className="flex items-center gap-1"><CornerDownLeft className="w-2.5 h-2.5" /> ENTER TO SEND</span>
        </div>
      </div>
    </div>
  );
}
