import nodemailer from "nodemailer";
import dotenv from 'dotenv';
dotenv.config();


export const sendErrorEmail = async(error: any) => {
    console.log("Sending error email...");
    
    const transporter = nodemailer.createTransport({
      host: process.env.EMAIL_HOST,
      port: parseInt(process.env.EMAIL_PORT || ""),
      secure: true,
      auth: {
        user: process.env.EMAIL_USER,
        pass: process.env.EMAIL_PASS,
      },
    });
  
    await transporter.sendMail({
      from: `"Cron Error" <${process.env.EMAIL_USER}>`,
      to: process.env.EMAIL_TO,
      subject: "Daily Task Failed",
      text: error.stack || error.message || String(error),
    });
  }
  
export const sendSuccessEmail = async() => {
    console.log("Sending success email...");
  
    const transporter = nodemailer.createTransport({
      host: process.env.EMAIL_HOST,
      port: parseInt(process.env.EMAIL_PORT || ""),
      secure: true,
      auth: {
        user: process.env.EMAIL_USER,
        pass: process.env.EMAIL_PASS,
      },
    });
  
    await transporter.sendMail({
      from: `"Cron Success" <${process.env.EMAIL_USER}>`,
      to: process.env.EMAIL_TO,
      subject: "Daily Prices Updated Successfully",
      text: "Daily prices have been updated successfully.",
    });
  }
  