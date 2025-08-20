import { PinataSDK } from "pinata";  

async function testUpload() {  
  const pinata = new PinataSDK({  
    pinataJwt: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiIzMTljMzg0OC02YWM4LTRkZDEtOWY1NS03OTgxZTgyODc4Y2IiLCJlbWFpbCI6ImVsZWJ1cnVpa2VoYXNzYW5AZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsInBpbl9wb2xpY3kiOnsicmVnaW9ucyI6W3siZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjEsImlkIjoiRlJBMSJ9LHsiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjEsImlkIjoiTllDMSJ9XSwidmVyc2lvbiI6MX0sIm1mYV9lbmFibGVkIjpmYWxzZSwic3RhdHVzIjoiQUNUSVZFIn0sImF1dGhlbnRpY2F0aW9uVHlwZSI6InNjb3BlZEtleSIsInNjb3BlZEtleUtleSI6IjY0MWRjOTM3ODdlYTQzMzcyODAyIiwic2NvcGVkS2V5U2VjcmV0IjoiYzM2M2QyNjhiN2Y5N2RmOGM2NTYzM2M5NzM5NTk5NzFhN2VlYjFmNTc1YTc3YTUyZmJlYzQ5NDJlZGEyNjVlOCIsImV4cCI6MTc4NjYxOTIwOH0.QPKczQpJF8TE69_m4tiddJHtgReANEkRA1l6Wqkg90g", 
    pinataGateway: "brown-acceptable-butterfly-89.mypinata.cloud",  
  });  

  try {  
    const file = new File(["Hello, NebulaForge from SDK!"], "test.txt", { type: "text/plain" });  
    const upload = await pinata.upload.public.file(file);  
    console.log("Upload successful:", upload.cid);  
    console.log("Access: https://gateway.pinata.cloud/ipfs/" + upload.cid);  
  } catch (error) {  
    console.error("Upload failed:", error);
  }  
}  

testUpload();  