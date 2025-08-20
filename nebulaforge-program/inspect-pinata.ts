import { PinataSDK } from "pinata";
const pinata = new PinataSDK({ pinataJwt: "dummy", pinataGateway: "dummy" });
console.log(pinata.upload); // Log the upload object