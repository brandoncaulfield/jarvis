import express, { Request, Response, NextFunction } from "express";
import bodyParser from "body-parser";
import axios from "axios";
import * as dotenv from "dotenv";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;
const huggingfaceEndpoint =
  process.env.HUGGINGFACE_ENDPOINT ||
  "https://api-inference.huggingface.co/models/tiiuae/falcon-7b-instruct";
const huggingfaceApiKey = process.env.HUGGINGFACE_API_KEY;

app.use(bodyParser.json());

app.post(
  "/generate",
  async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { prompt } = req.body;

      if (!prompt || typeof prompt !== "string") {
        return res.status(400).json({
          error: 'Invalid input. "prompt" must be a non-empty string.',
        });
      }

      // Set up Hugging Face API call
      const huggingFaceUrl = huggingfaceEndpoint;
      const headers = {
        Authorization: `Bearer ${huggingfaceApiKey}`,
      };

      const response = await axios.post(
        huggingFaceUrl,
        {
          inputs: prompt,
          parameters: {
            do_sample: true,
            top_p: 0.9,
            temperature: 0.8,
            max_new_tokens: 1024,
            repetition_penalty: 1.03,
            stop: ["\nUser:", "<|endoftext|>", "</s>"],
          },
        },
        { headers }
      );
      const generatedText = response.data[0].generated_text;

      res.json({ generatedText });
    } catch (error) {
      console.error("Error generating text:", error);
      res
        .status(500)
        .json({ error: "An error occurred while generating text." });
    }
  }
);

// Error handler for invalid routes
app.use((req: Request, res: Response) => {
  res.status(404).json({ error: "Route not found" });
});

// Global error handler
app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error("Global error handler:", err);
  res.status(500).json({ error: "Something went wrong" });
});

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
