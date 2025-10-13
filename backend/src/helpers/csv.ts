import * as fs from 'fs';
import * as https from 'https';
import { parse } from 'csv-parse';
import * as path from 'path';
import { promisify } from 'util';

const readFile = promisify(fs.readFile);
const unlink = promisify(fs.unlink);

/**
 * Downloads a file from a URL and saves it to the given path (in root directory).
 */
export async function downloadFile(url: string, destFileName: string): Promise<void> {
  const destPath = path.resolve(__dirname, '..', destFileName);

  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(destPath);
    console.log("Getting URL:", url);

    https.get(url, (response) => {
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to get file: ${response.statusCode}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => file.close((err?: NodeJS.ErrnoException | null) => {
        if (err) reject(err);
        else resolve();
      }));
    }).on('error', async (err) => {
      await unlink(destPath).catch(() => {});
      reject(err);
    });
  });
}


/**
 * Parses a semicolon-delimited CSV file in the root folder into JSON objects.
 */
export async function parseCsv(fileName: string): Promise<Record<string, string>[]> {
  const filePath = path.resolve(__dirname, '..', fileName);
  const fileContent = await readFile(filePath, 'utf-8');

  return new Promise((resolve, reject) => {
    parse(fileContent, {
      delimiter: ';',
      trim: true,
      skip_empty_lines: true,
      relax_column_count: true,
      columns: (header: string[]) =>
        header
          .map(h => h.replace(/^\uFEFF/, '').trim())
          .filter(h => h !== ''),
    }, (err, records: Record<string, string>[]) => {
      if (err) reject(err);
      else resolve(records);
    });
  });
}
