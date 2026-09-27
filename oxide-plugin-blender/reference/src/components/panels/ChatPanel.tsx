import React, { useState, useRef, useEffect } from 'react';
import { Send, Sparkles, AlertCircle, ArrowRight } from 'lucide-react';
import { useEnclosureStore } from '../../state/enclosureStore';

interface Message {
  sender: 'user' | 'assistant';
  text: string;
  timestamp: string;
  proposedParams?: any;
}

const SUGGESTED_PROMPTS = [
  "Optimize design for Raspberry Pi 5 thermal airflow",
  "Adjust enclosure to be compact and lightweight",
  "Design a heavy-duty Aluminum heat sink casing",
  "Add moderate ventilation array with high spacing"
];

export function ChatPanel() {
  const { currentEnclosure, updateParams } = useEnclosureStore();
  const [messages, setMessages] = useState<Message[]>([
    {
      sender: 'assistant',
      text: "Hello! I am your Blender CAD integration assistant. Tell me what kind of electronics you are housing (e.g., Raspberry Pi, custom PCB, Arduino) or the environmental constraints, and I will recommend optimal parametric dimensions and ventilation for your enclosure. 🔧✨",
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    }
  ]);
  const [input, setInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, isTyping]);

  const parseJsonBlock = (text: string) => {
    try {
      // Find JSON block matches
      const re = /```json\s*(\{[\s\S]*?\})\s*```/;
      const match = text.match(re);
      if (match && match[1]) {
        return JSON.parse(match[1]);
      }
    } catch (e) {
      console.warn("Error parsing proposed JSON from assistant message:", e);
    }
    return null;
  };

  const handleSend = async (textToSend: string) => {
    if (!textToSend.trim()) return;

    const userMessage: Message = {
      sender: 'user',
      text: textToSend,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setIsTyping(true);

    try {
      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          prompt: textToSend,
          currentParams: currentEnclosure
        })
      });

      const data = await response.json();
      
      let proposed = null;
      if (data.text) {
        proposed = parseJsonBlock(data.text);
      }

      const assistantMessage: Message = {
        sender: 'assistant',
        text: data.text || "I was unable to retrieve a response. Please verify the Gemini API configuration.",
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        proposedParams: proposed
      };

      setMessages((prev) => [...prev, assistantMessage]);
    } catch (err: any) {
      const errorMessage: Message = {
        sender: 'assistant',
        text: `Error calling AI client: ${err.message || 'Unknown network error. Is the server running?'}`,
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsTyping(false);
    }
  };

  const applyProposedParams = (params: any) => {
    if (!params) return;
    updateParams({
      ...currentEnclosure,
      dimensions: params.dimensions ? { ...currentEnclosure.dimensions, ...params.dimensions } : currentEnclosure.dimensions,
      wallThickness: params.wallThickness !== undefined ? params.wallThickness : currentEnclosure.wallThickness,
      material: params.material !== undefined ? params.material : currentEnclosure.material,
      ventConfig: params.ventConfig ? { ...currentEnclosure.ventConfig, ...params.ventConfig } : currentEnclosure.ventConfig
    });
  };

  return (
    <div className="flex flex-col h-full bg-[#18181b] border-l border-[#27272a] rounded-2xl w-80 shrink-0 overflow-hidden text-[#fafafa]">
      {/* Header */}
      <div className="p-4 border-b border-[#27272a] shrink-0">
        <div className="flex items-center gap-2">
          <div className="bg-[#27272a] p-1.5 rounded-lg border border-[#3f3f46]">
            <Sparkles size={14} className="text-white" />
          </div>
          <div>
            <h3 className="text-xs font-bold tracking-tight text-white leading-none">AI CAD Copilot</h3>
            <p className="text-[10px] text-[#a1a1aa] font-mono mt-0.5">Blender Local Assistant Model</p>
          </div>
        </div>
      </div>

      {/* Message Feed */}
      <div className="flex-grow overflow-y-auto p-4 space-y-3 flex flex-col">
        {messages.map((msg, idx) => (
          <div
            key={idx}
            className={`flex flex-col max-w-[90%] ${
              msg.sender === 'user' ? 'self-end items-end' : 'self-start items-start'
            }`}
          >
            {/* Sender Label */}
            <span className="text-[9px] uppercase tracking-wider font-mono text-[#71717a] mb-1">
              {msg.sender === 'user' ? 'User' : 'Assistant'} • {msg.timestamp}
            </span>
            
            {/* Message Bubble */}
            <div
              className={`p-3 rounded-xl text-xs leading-relaxed border ${
                msg.sender === 'user'
                  ? 'bg-white text-black font-medium border-white rounded-tr-none'
                  : 'bg-[#09090b] text-[#fafafa] border-[#27272a] rounded-tl-none'
              }`}
            >
              <div className="whitespace-pre-wrap select-text">{msg.text}</div>

              {/* Special interactive proposal block */}
              {msg.proposedParams && (
                <div className="mt-3 p-2.5 bg-white/5 border border-[#27272a] rounded-lg flex flex-col gap-1.5">
                  <div className="flex items-center gap-1 text-[10px] text-white uppercase tracking-wider font-bold">
                    <Sparkles size={11} className="text-yellow-400" />
                    Optimal Params Found
                  </div>
                  <div className="grid grid-cols-2 gap-x-1.5 gap-y-0.5 font-mono text-[10px] text-[#a1a1aa]">
                    {msg.proposedParams.dimensions && (
                      <div>
                        Size: {msg.proposedParams.dimensions.width}×
                        {msg.proposedParams.dimensions.height}×
                        {msg.proposedParams.dimensions.depth}
                      </div>
                    )}
                    {msg.proposedParams.wallThickness !== undefined && (
                      <div>Wall: {msg.proposedParams.wallThickness}mm</div>
                    )}
                    {msg.proposedParams.ventConfig && (
                      <div className="col-span-2">
                        Vents: {msg.proposedParams.ventConfig.quantity}×
                        {msg.proposedParams.ventConfig.holeDiameter}
                      </div>
                    )}
                  </div>
                  <button
                    onClick={() => applyProposedParams(msg.proposedParams)}
                    className="mt-1 flex items-center justify-center gap-1 text-[10px] px-2 py-1 bg-white text-black rounded-md hover:bg-gray-200 transition font-bold"
                  >
                    Apply Parameters <ArrowRight size={11} />
                  </button>
                </div>
              )}
            </div>
          </div>
        ))}
        {isTyping && (
          <div className="self-start max-w-[90%] flex flex-col items-start">
            <span className="text-[9px] uppercase tracking-wider font-mono text-[#71717a] mb-1">
              Assistant • Thinking...
            </span>
            <div className="bg-[#09090b] border border-[#27272a] p-2 rounded-xl rounded-tl-none text-xs text-[#a1a1aa] flex items-center gap-1.5">
              <span className="flex gap-0.5">
                <span className="w-1 h-1 bg-[#71717a] rounded-full animate-bounce delay-100"></span>
                <span className="w-1 h-1 bg-[#71717a] rounded-full animate-bounce delay-200"></span>
                <span className="w-1 h-1 bg-[#71717a] rounded-full animate-bounce delay-300"></span>
              </span>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Suggested prompts list */}
      {messages.length === 1 && (
        <div className="px-4 pb-2 space-y-1.5">
          <p className="text-[9px] uppercase tracking-widest font-mono text-[#71717a]">Suggested Prompts</p>
          <div className="flex flex-col gap-1 max-h-40 overflow-y-auto pr-1">
            {SUGGESTED_PROMPTS.map((promptText, idx) => (
              <button
                key={idx}
                onClick={() => handleSend(promptText)}
                className="text-left text-[10px] bg-[#09090b] hover:bg-[#27272a] border border-[#27272a] text-[#a1a1aa] hover:text-[#fafafa] p-1.5 rounded-lg transition-all font-medium leading-normal"
              >
                {promptText}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Input container */}
      <div className="p-4 border-t border-[#27272a] shrink-0">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            handleSend(input);
          }}
          className="flex gap-1.5 items-center"
        >
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Ask AI assistant..."
            className="flex-grow px-3 py-1.5 bg-[#09090b] border border-[#27272a] rounded-lg text-xs focus:outline-none focus:border-white text-white font-medium placeholder-[#71717a] transition-all"
          />
          <button
            type="submit"
            className="p-2 bg-white text-black hover:bg-gray-200 rounded-lg transition duration-150 shadow"
          >
            <Send size={13} />
          </button>
        </form>
      </div>
    </div>
  );
}
