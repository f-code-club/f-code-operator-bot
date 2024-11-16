const { Client, IntentsBitField } = require("discord.js");
const ExcelJS = require("exceljs");
require("dotenv").config();

// Initialize Discord client
const client = new Client({
  intents: [
    IntentsBitField.Flags.Guilds,
    IntentsBitField.Flags.GuildMembers,
    IntentsBitField.Flags.GuildMessages,
    IntentsBitField.Flags.MessageContent,
  ],
});

// Constants
const prefix = process.env.DISCORD_PREFIX;
const token = process.env.DISCORD_TOKEN;
const excelFilePath = "src/students.csv";
const validStudentCodeRegex = /^[A-Za-z]{2}\d{6}$/;
const roleName = "Round 1: Challenger";
const formattedDate = new Intl.DateTimeFormat("vi-VN", {
  year: "numeric",
  month: "2-digit",
  day: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
});

client.on("messageCreate", async (message) => {
  if (message.author.bot) return;

  if (message.content.startsWith(`${prefix}CHECK_`)) {
    const studentCode = message.content.slice(7);

    if (validStudentCodeRegex.test(studentCode)) {
      try {
        const workbook = new ExcelJS.Workbook();
        await workbook.csv.readFile(excelFilePath);
        const worksheet = workbook.getWorksheet(1);
        const member = message.guild.members.cache.get(message.author.id);

        if (!member.displayName.toUpperCase().includes(studentCode)) {
          return message.reply("Vui lòng đặt tên đúng quy tắc.");
        }

        let foundRow = null;
        worksheet.eachRow((row) => {
          if (
            row.getCell("A").value.toUpperCase() === studentCode.toUpperCase()
          ) {
            foundRow = row;
          }
        });

        if (foundRow) {
          if (foundRow.getCell("B").value === 0) {
            foundRow.getCell("B").value = "1";
            foundRow.getCell("C").value = new Date().toString();
            await workbook.csv.writeFile(excelFilePath);

            const role = message.guild.roles.cache.find(
              (r) => r.name === roleName
            );

            if (role && member) {
              await member.roles.add(role);
              return message.reply("Đã thêm role cho bạn.");
            } else {
              return message.reply("Không tìm thấy thành viên.");
            }
          } else {
            return message.reply(
              `Bạn đã xác nhận thành công vào lúc ${formattedDate.format(
                new Date(foundRow.getCell("C").value)
              )}`
            );
          }
        } else {
          return message.reply("Mã số sinh viên này không tồn tại.");
        }
      } catch (error) {
        console.error("Error accessing Excel file:", error);
        return message.reply(
          "Có một số lỗi đã xảy ra bạn thử lại trong ít phút hoặc tạo ticket để được hỗ trợ nhé!"
        );
      }
    } else {
      return message.reply("Mã số sinh viên không hợp lệ.");
    }
  }
});

client.on("ready", () => {
  console.log(`Bot đã sẵn sàng với tên: ${client.user.tag}`);
});

client.login(token);
