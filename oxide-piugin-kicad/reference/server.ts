import express from "express";
import path from "path";
import { createServer as createViteServer } from "vite";
import { GoogleGenAI, Type } from "@google/genai";
import dotenv from "dotenv";

dotenv.config();

const ai = new GoogleGenAI({
  apiKey: process.env.GEMINI_API_KEY,
  httpOptions: {
    headers: {
      'User-Agent': 'aistudio-build',
    }
  }
});

async function startServer() {
  const app = express();
  const PORT = 3000;

  app.use(express.json());

  // API Route for chat assistant
  app.post("/api/chat", async (req, res) => {
    try {
      const { message, history } = req.body;
      if (!message) {
        return res.status(400).json({ error: "Message is required" });
      }

      const modelName = 'gemini-3.5-flash';

      // Build context structure with history
      const contents = [];
      if (history && Array.isArray(history)) {
        for (const item of history) {
          contents.push({
            role: item.role === 'user' ? 'user' : 'model',
            parts: [{ text: item.text }]
          });
        }
      }
      contents.push({
        role: 'user',
        parts: [{ text: message }]
      });

      const response = await ai.models.generateContent({
        model: modelName,
        contents: contents,
        config: {
          systemInstruction: "You are an expert EDA/PCB design assistant inside an elegant dark-themed EDA suite. Help users programmatically generate circuits, analyze netlists, review design rules, write SKiDL, or optimize track routing. Be concise, precise, and professional. Avoid markdown wrappers or blocks if answering simple commands, and output helpful hints about the workspace when asked.",
        }
      });

      return res.json({ text: response.text });
    } catch (err: any) {
      console.error("Gemini API error:", err);
      return res.status(500).json({ error: err.message || "Failed to query the AI assistant." });
    }
  });

  // API Route to parse component datasheets into precise CAD JSON schemas (Schematic symbol, PCB footprint, and 3D attributes)
  app.post("/api/datasheet-parser", async (req, res) => {
    try {
      const { datasheetText } = req.body;
      if (!datasheetText) {
        return res.status(400).json({ error: "Datasheet text/specs are required" });
      }

      const response = await ai.models.generateContent({
        model: "gemini-3.5-flash",
        contents: `Analyze the following datasheet information or pinout block, and extract/synthesize the complete component properties, schematic pins, and PCB footprint dimensions (preferably standard layout SMD or DIP depending on the specified package). Spec:\n\n${datasheetText}`,
        config: {
          systemInstruction: "You are a professional silicon verification and component librarian. Extract mechanical packaging dimensions, pinouts, and electrical functions. Synthesize coordinates for footprint pads intelligently based on the package (e.g. DIP, SOIC, SOT, QFP). If dimensions are not explicitly listed, make expert approximations based on the standard package type (e.g. DIP pitch is 2.54mm, SOIC pitch is 1.27mm). Keep dimensions in millimeters.",
          responseMimeType: "application/json",
          responseSchema: {
            type: Type.OBJECT,
            properties: {
              name: { type: Type.STRING, description: "Official part name, e.g. NE555, LM358, ESP32-WROOM-32" },
              value: { type: Type.STRING, description: "Functional description, e.g. 8-Bit MCU, Linear LDO Regulator, Op-Amp" },
              type: { type: Type.STRING, description: "General component type, e.g. MCU, Regulator, Op-Amp, Sensor, Connector" },
              referencePrefix: { type: Type.STRING, description: "Standard schematic designator prefix, e.g. U, J, Q" },
              package: { type: Type.STRING, description: "Synthesized package type, e.g. DIP-8, SOIC-8, SOT-23-5, QFP-32, ESP32" },
              pins: {
                type: Type.ARRAY,
                items: {
                  type: Type.OBJECT,
                  properties: {
                    num: { type: Type.STRING, description: "Pin ID number, e.g. '1', '2'" },
                    name: { type: Type.STRING, description: "Pin formal name, e.g. 'GND', 'VCC', 'RXD'" },
                    type: { type: Type.STRING, description: "Electrical type: input, output, power, gnd, passive" }
                  },
                  required: ["num", "name", "type"]
                }
              },
              dimensions: {
                type: Type.OBJECT,
                properties: {
                  width: { type: Type.NUMBER, description: "Recommended CAD footprint grid width limit in mm, e.g. 10.0" },
                  height: { type: Type.NUMBER, description: "Recommended CAD footprint grid height limit in mm, e.g. 8.0" },
                  pitch: { type: Type.NUMBER, description: "Spacing index between adjacent pins in mm, e.g. 1.27 or 2.54" },
                  bodyWidth: { type: Type.NUMBER, description: "Mechanical body X-width dimensions in mm, e.g. 6.3" },
                  bodyLength: { type: Type.NUMBER, description: "Mechanical body Y-length dimensions in mm, e.g. 9.3" },
                  bodyHeight: { type: Type.NUMBER, description: "Mechanical Z-envelope height limits in mm, e.g. 2.5" },
                  color: { type: Type.STRING, description: "Hex value for the package body representation, e.g. '#1e1e24'" }
                },
                required: ["width", "height", "pitch", "bodyWidth", "bodyLength", "bodyHeight", "color"]
              }
            },
            required: ["name", "value", "type", "referencePrefix", "package", "pins", "dimensions"]
          }
        }
      });

      const parsedData = JSON.parse(response.text || "{}");
      return res.json(parsedData);
    } catch (err: any) {
      console.error("Datasheet parser API error:", err);
      return res.status(500).json({ error: err.message || "Failed to parse datasheet." });
    }
  });

  // Vite middleware for development
  if (process.env.NODE_ENV !== "production") {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    // In production, serve the compiled assets from dist
    const distPath = path.join(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*', (req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
