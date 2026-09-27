import express from "express";
import path from "path";
import { createServer as createViteServer } from "vite";
import { GoogleGenAI } from "@google/genai";
import dotenv from "dotenv";

dotenv.config();

async function startServer() {
  const app = express();
  const PORT = 3000;

  app.use(express.json());

  // API endpoint for chat
  app.post("/api/chat", async (req, res) => {
    try {
      const { prompt, currentParams } = req.body;
      
      const apiKey = process.env.GEMINI_API_KEY;
      if (!apiKey || apiKey === "MY_GEMINI_API_KEY" || apiKey.trim() === "") {
        return res.json({
          text: `Hello! I would love to help you design your enclosure. However, the **GEMINI_API_KEY** is not configured.\n\nPlease set your key in the **Secrets** panel in Google AI Studio to start using the smart CAD prompt design assistant! 💻🔧`,
          error: true
        });
      }

      const ai = new GoogleGenAI({ 
        apiKey: apiKey,
        httpOptions: {
          headers: {
            'User-Agent': 'aistudio-build',
          }
        }
      });
      
      const systemInstruction = `You are an expert mechanical design AI Assistant integrated directly into the Tauri CAD Control Plane for Blender.
Your goal is to help user design parametric 3D printed/milled electronics enclosures.
The user is working with the following parameters:
- Width: ${currentParams?.dimensions?.width || 100}mm
- Height: ${currentParams?.dimensions?.height || 80}mm
- Depth: ${currentParams?.dimensions?.depth || 60}mm
- Wall Thickness: ${currentParams?.wallThickness || 2.5}mm
- Material: ${currentParams?.material || 'pla'}
- Vent hole diameter: ${currentParams?.ventConfig?.holeDiameter || 3}mm
- Vent hole spacing: ${currentParams?.ventConfig?.spacing || 5}mm
- Vent hole quantity: ${currentParams?.ventConfig?.quantity || 12}

You can suggest specific design changes. To apply parameters automatically, you can respond with a JSON block in your answer containing the new exact parameters so the frontend can parse it and update the model instantly!
JSON block format:
\`\`\`json
{
  "dimensions": { "width": 120, "height": 90, "depth": 50 },
  "wallThickness": 3.0,
  "material": "pla",
  "ventConfig": { "holeDiameter": 4, "spacing": 6, "quantity": 16 }
}
\`\`\`
Make sure any parameters you propose are mathematically valid:
- Width, height, depth must be between 50mm and 500mm
- Wall thickness between 1.5mm and 10mm
- Vent hole diameter between 2mm and 50mm

Keep your response friendly, concise, and professional. Explain *why* you suggest the sizes (e.g., thermal airflow, mechanical strength, structure fitting, material efficiency, PLA vs Aluminum).`;

      const response = await ai.models.generateContent({
        model: "gemini-3.5-flash",
        contents: prompt,
        config: {
          systemInstruction,
        }
      });

      res.json({ text: response.text });
    } catch (err: any) {
      console.error(err);
      res.status(500).json({ error: true, text: `Server side error processing model: ${err.message}` });
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
    const distPath = path.join(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*', (req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://0.0.0.0:${PORT}`);
  });
}

startServer();
