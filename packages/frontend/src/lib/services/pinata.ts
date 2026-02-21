/**
 * Pinata IPFS upload service
 *
 * In production: Uses Netlify serverless function to keep JWT secure
 * In development: Uses VITE_PINATA_JWT env var directly
 */

export interface UploadResult {
  cid: string;
  pinSize: number;
  date: string;
}

export interface FileValidation {
  valid: boolean;
  error?: string;
}

// Pinata free tier max file size: 100MB
const MAX_FILE_SIZE = 100 * 1024 * 1024;

// Allowed file types for work evidence
const ALLOWED_TYPES = [
  // Images
  'image/jpeg',
  'image/png',
  'image/gif',
  'image/webp',
  'image/heic',
  'image/heif',
  // Documents
  'application/pdf',
  'application/msword',
  'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  // Video
  'video/mp4',
  'video/quicktime',
  'video/webm',
  // Audio
  'audio/mpeg',
  'audio/wav',
  'audio/ogg',
];

/**
 * Validate file before upload
 */
export function validateFile(file: File): FileValidation {
  if (!file) {
    return { valid: false, error: 'No file selected' };
  }

  if (file.size > MAX_FILE_SIZE) {
    const sizeMB = (file.size / (1024 * 1024)).toFixed(1);
    return { valid: false, error: `File too large (${sizeMB}MB). Maximum is 100MB.` };
  }

  // Allow any file type for flexibility, but warn about non-standard types
  // In practice, most evidence will be photos/documents
  return { valid: true };
}

/**
 * Check if file type is recommended for work evidence
 */
export function isRecommendedType(file: File): boolean {
  return ALLOWED_TYPES.includes(file.type);
}

/**
 * Get human-readable file type
 */
export function getFileTypeName(file: File): string {
  const type = file.type;

  if (type.startsWith('image/')) return 'Image';
  if (type.startsWith('video/')) return 'Video';
  if (type.startsWith('audio/')) return 'Audio';
  if (type === 'application/pdf') return 'PDF';
  if (type.includes('word')) return 'Document';
  if (type.includes('spreadsheet') || type.includes('excel')) return 'Spreadsheet';

  return 'File';
}

/**
 * Format file size for display
 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

/**
 * Upload file to IPFS via Pinata
 *
 * In production (Netlify): Uses serverless function at /.netlify/functions/upload-to-ipfs
 * In development: Uses direct Pinata API with VITE_PINATA_JWT env var
 */
export async function uploadToIPFS(file: File, onProgress?: (percent: number) => void): Promise<UploadResult> {
  // Check if we're in production (Netlify) or development
  const isProduction = typeof window !== 'undefined' &&
    !window.location.hostname.includes('localhost') &&
    !window.location.hostname.includes('127.0.0.1');

  const formData = new FormData();
  formData.append('file', file);

  // Add metadata
  const metadata = JSON.stringify({
    name: `work-evidence-${Date.now()}`,
    keyvalues: {
      app: 'kachi.ng',
      type: 'work-evidence',
      originalName: file.name,
    }
  });
  formData.append('pinataMetadata', metadata);

  // Add options
  const options = JSON.stringify({
    cidVersion: 1,
  });
  formData.append('pinataOptions', options);

  if (isProduction) {
    // Use Netlify serverless function (JWT is securely stored in env)
    return uploadViaFunction(formData, onProgress);
  } else {
    // Development: Use direct API with VITE_PINATA_JWT
    const jwt = import.meta.env.VITE_PINATA_JWT;
    if (!jwt) {
      throw new Error('VITE_PINATA_JWT environment variable not set. Get a JWT from pinata.cloud and add it to your .env file.');
    }
    return uploadDirect(formData, jwt, onProgress);
  }
}

/**
 * Upload via Netlify serverless function (production)
 */
async function uploadViaFunction(formData: FormData, onProgress?: (percent: number) => void): Promise<UploadResult> {
  // Note: Progress tracking not possible with fetch + serverless function
  // We'll just show indeterminate progress
  onProgress?.(50);

  const response = await fetch('/.netlify/functions/upload-to-ipfs', {
    method: 'POST',
    body: formData,
  });

  onProgress?.(100);

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Upload failed' }));
    throw new Error(error.error || `Upload failed: ${response.status}`);
  }

  const result = await response.json();
  return {
    cid: result.IpfsHash || result.cid,
    pinSize: result.PinSize || result.pinSize || 0,
    date: result.Timestamp || result.date || new Date().toISOString(),
  };
}

/**
 * Upload directly to Pinata API (development)
 */
async function uploadDirect(formData: FormData, jwt: string, onProgress?: (percent: number) => void): Promise<UploadResult> {
  // Note: Progress tracking with fetch requires streaming, which Pinata doesn't fully support
  // We'll simulate progress for UX
  onProgress?.(30);

  const response = await fetch('https://api.pinata.cloud/pinning/pinFileToIPFS', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${jwt}`,
    },
    body: formData,
  });

  onProgress?.(100);

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: { reason: 'Upload failed' } }));
    throw new Error(error.error?.reason || error.error || `Upload failed: ${response.status}`);
  }

  const result = await response.json();
  return {
    cid: result.IpfsHash,
    pinSize: result.PinSize,
    date: result.Timestamp,
  };
}

/**
 * Get IPFS gateway URL for viewing a file
 */
export function getIPFSUrl(cid: string, gateway: string = 'gateway.pinata.cloud'): string {
  // Support both CIDv0 (Qm...) and CIDv1 (bafy...)
  return `https://${gateway}/ipfs/${cid}`;
}

/**
 * Get common IPFS gateway URLs for a CID
 */
export function getGatewayUrls(cid: string): { name: string; url: string }[] {
  return [
    { name: 'Pinata', url: getIPFSUrl(cid, 'gateway.pinata.cloud') },
    { name: 'IPFS.io', url: getIPFSUrl(cid, 'ipfs.io') },
    { name: 'dWeb', url: getIPFSUrl(cid, 'dweb.link') },
  ];
}

/**
 * Check if a string looks like a valid IPFS CID
 */
export function isValidCID(cid: string): boolean {
  if (!cid || typeof cid !== 'string') return false;

  // CIDv0: starts with Qm, 46 characters, base58
  if (/^Qm[1-9A-HJ-NP-Za-km-z]{44}$/.test(cid)) return true;

  // CIDv1: starts with b, variable length, base32 or base58
  if (/^b[a-z2-7]{58,}$/.test(cid)) return true;
  if (/^b[A-Za-z0-9]{58,}$/.test(cid)) return true;

  return false;
}
