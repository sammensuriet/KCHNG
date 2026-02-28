/**
 * Netlify serverless function for secure IPFS upload via Pinata
 *
 * This function:
 * 1. Receives file uploads from the frontend
 * 2. Forwards them to Pinata API with the secret JWT
 * 3. Returns the IPFS CID to the client
 *
 * The JWT is stored securely in Netlify environment variables,
 * never exposed to the browser.
 */

import type { Handler } from '@netlify/functions';

const PINATA_API_URL = 'https://api.pinata.cloud/pinning/pinFileToIPFS';

export const handler: Handler = async (event) => {
  // Only allow POST requests
  if (event.httpMethod !== 'POST') {
    return {
      statusCode: 405,
      body: JSON.stringify({ error: 'Method not allowed' }),
    };
  }

  // Check for JWT
  const jwt = process.env.PINATA_JWT;
  if (!jwt) {
    console.error('PINATA_JWT environment variable not set');
    return {
      statusCode: 500,
      body: JSON.stringify({ error: 'Server configuration error' }),
    };
  }

  try {
    // Parse multipart form data
    const boundary = extractBoundary(event.headers['content-type']);
    if (!boundary) {
      return {
        statusCode: 400,
        body: JSON.stringify({ error: 'Invalid content type' }),
      };
    }

    const body = event.isBase64Encoded
      ? Buffer.from(event.body || '', 'base64')
      : event.body || '';

    // Forward to Pinata
    const response = await fetch(PINATA_API_URL, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${jwt}`,
      },
      body: buildFormData(body, boundary),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error('Pinata API error:', response.status, errorText);

      let errorMessage = 'Upload failed';
      try {
        const errorJson = JSON.parse(errorText);
        errorMessage = errorJson.error?.reason || errorJson.error || errorMessage;
      } catch {
        // Use default error message
      }

      return {
        statusCode: response.status,
        body: JSON.stringify({ error: errorMessage }),
      };
    }

    const result = await response.json();

    // Return successful response
    return {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        IpfsHash: result.IpfsHash,
        PinSize: result.PinSize,
        Timestamp: result.Timestamp,
      }),
    };
  } catch (error) {
    console.error('Upload error:', error);
    return {
      statusCode: 500,
      body: JSON.stringify({
        error: error instanceof Error ? error.message : 'Upload failed'
      }),
    };
  }
};

/**
 * Extract boundary from content-type header
 */
function extractBoundary(contentType: string | undefined): string | null {
  if (!contentType) return null;

  const match = contentType.match(/boundary=(.+)/);
  return match ? match[1].trim().replace(/"/g, '') : null;
}

/**
 * Build FormData for Pinata API from parsed body
 */
function buildFormData(body: string | Buffer, boundary: string): Buffer {
  const CRLF = '\r\n';
  const parts: Buffer[] = [];

  // Parse the multipart data and extract file
  const bodyStr = typeof body === 'string' ? body : body.toString('binary');
  const boundaryBuffer = `--${boundary}`;

  // Split by boundary
  const sections = bodyStr.split(boundaryBuffer);

  for (const section of sections) {
    if (section.trim() === '' || section.trim() === '--') continue;

    // Find header/body separator
    const headerEndIndex = section.indexOf(CRLF + CRLF);
    if (headerEndIndex === -1) continue;

    const headers = section.substring(0, headerEndIndex);
    let content = section.substring(headerEndIndex + 4);

    // Remove trailing boundary markers and CRLF
    content = content.replace(/\r\n--$/, '').replace(/\r\n$/, '');

    // Only include the file field, skip other fields (we'll add our own metadata)
    if (headers.includes('name="file"')) {
      // Build the part
      parts.push(Buffer.from(`--${boundary}${CRLF}`, 'binary'));
      parts.push(Buffer.from(headers + CRLF + CRLF, 'binary'));

      // Handle binary content
      if (typeof body === 'string') {
        parts.push(Buffer.from(content, 'binary'));
      } else {
        // Extract binary content from original buffer
        const startIdx = bodyStr.indexOf(section.substring(0, 100));
        if (startIdx !== -1) {
          const contentStart = startIdx + headerEndIndex + 4;
          const contentEnd = contentStart + content.length;
          parts.push(body.slice(contentStart, contentEnd));
        }
      }
    }
  }

  // Add our own metadata
  const metadata = JSON.stringify({
    name: `work-evidence-${Date.now()}`,
    keyvalues: {
      app: 'kchng.org',
      type: 'work-evidence',
      uploadedAt: new Date().toISOString(),
    }
  });

  parts.push(Buffer.from(`${CRLF}--${boundary}${CRLF}`, 'binary'));
  parts.push(Buffer.from(
    `Content-Disposition: form-data; name="pinataMetadata"${CRLF}${CRLF}${metadata}`,
    'binary'
  ));

  // Add options
  const options = JSON.stringify({ cidVersion: 1 });
  parts.push(Buffer.from(`${CRLF}--${boundary}${CRLF}`, 'binary'));
  parts.push(Buffer.from(
    `Content-Disposition: form-data; name="pinataOptions"${CRLF}${CRLF}${options}`,
    'binary'
  ));

  parts.push(Buffer.from(`${CRLF}--${boundary}--${CRLF}`, 'binary'));

  return Buffer.concat(parts);
}
