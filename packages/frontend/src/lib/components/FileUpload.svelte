<script lang="ts">
  import {
    uploadToIPFS,
    validateFile,
    isRecommendedType,
    getFileTypeName,
    formatFileSize,
    getIPFSUrl,
    isValidCID,
  } from '$lib/services/pinata';

  interface Props {
    onUpload: (cid: string) => void;
    existingCid?: string;
    disabled?: boolean;
  }

  let { onUpload, existingCid = '', disabled = false }: Props = $props();

  // State
  let isDragging = $state(false);
  let isUploading = $state(false);
  let uploadProgress = $state(0);
  let uploadedCid = $state(existingCid);
  let selectedFile = $state<File | null>(null);
  let previewUrl = $state<string | null>(null);
  let error = $state<string | null>(null);
  let showAdvanced = $state(false);
  let manualCid = $state('');

  // Check if we have an existing CID on mount
  $effect(() => {
    if (existingCid && isValidCID(existingCid)) {
      uploadedCid = existingCid;
    }
  });

  // Cleanup preview URL on unmount
  $effect(() => {
    return () => {
      if (previewUrl) {
        URL.revokeObjectURL(previewUrl);
      }
    };
  });

  // Computed
  const isImage = $derived(selectedFile?.type.startsWith('image/') ?? false);
  const isReadyToUpload = $derived(selectedFile !== null && !isUploading);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (!disabled) {
      isDragging = true;
    }
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (disabled) return;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      selectFile(files[0]);
    }
  }

  function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    const files = target.files;
    if (files && files.length > 0) {
      selectFile(files[0]);
    }
  }

  function selectFile(file: File) {
    error = null;

    // Validate file
    const validation = validateFile(file);
    if (!validation.valid) {
      error = validation.error || 'Invalid file';
      return;
    }

    // Warn about non-recommended types
    if (!isRecommendedType(file)) {
      // Still allow, but this is just FYI
      console.log('File type not in recommended list:', file.type);
    }

    selectedFile = file;

    // Create preview for images
    if (file.type.startsWith('image/')) {
      previewUrl = URL.createObjectURL(file);
    } else {
      previewUrl = null;
    }
  }

  async function handleUpload() {
    if (!selectedFile || isUploading) return;

    error = null;
    isUploading = true;
    uploadProgress = 0;

    try {
      const result = await uploadToIPFS(selectedFile, (percent) => {
        uploadProgress = percent;
      });

      uploadedCid = result.cid;
      onUpload(result.cid);

      // Clear selected file after successful upload
      selectedFile = null;
      if (previewUrl) {
        URL.revokeObjectURL(previewUrl);
        previewUrl = null;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Upload failed';
    } finally {
      isUploading = false;
      uploadProgress = 0;
    }
  }

  function handleRemoveFile() {
    selectedFile = null;
    if (previewUrl) {
      URL.revokeObjectURL(previewUrl);
      previewUrl = null;
    }
    error = null;
  }

  function handleClearUpload() {
    uploadedCid = '';
    onUpload('');
  }

  function handleManualCidSubmit() {
    if (isValidCID(manualCid)) {
      uploadedCid = manualCid;
      onUpload(manualCid);
      manualCid = '';
      showAdvanced = false;
    } else {
      error = 'Invalid IPFS CID format';
    }
  }

  function handleCameraCapture() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.capture = 'environment';
    input.onchange = (e) => {
      const target = e.target as HTMLInputElement;
      if (target.files && target.files.length > 0) {
        selectFile(target.files[0]);
      }
    };
    input.click();
  }
</script>

<div class="file-upload" class:disabled>
  {#if uploadedCid}
    <!-- Success State: Show uploaded CID -->
    <div class="upload-success">
      <div class="success-icon">✓</div>
      <div class="success-content">
        <div class="success-label">Evidence uploaded</div>
        <div class="cid-display">{uploadedCid}</div>
        <a
          href={getIPFSUrl(uploadedCid)}
          target="_blank"
          rel="noopener noreferrer"
          class="view-link"
        >
          View on IPFS →
        </a>
      </div>
      <button
        type="button"
        class="btn-clear"
        onclick={handleClearUpload}
        disabled={disabled}
      >
        Upload Different File
      </button>
    </div>

  {:else if selectedFile && !isUploading}
    <!-- Preview State: Show selected file before upload -->
    <div class="file-preview">
      {#if previewUrl}
        <img src={previewUrl} alt="Preview" class="preview-image" />
      {:else}
        <div class="preview-icon">
          {getFileTypeName(selectedFile)}
        </div>
      {/if}

      <div class="preview-info">
        <div class="preview-name">{selectedFile.name}</div>
        <div class="preview-size">{formatFileSize(selectedFile.size)}</div>
      </div>

      <div class="preview-actions">
        <button type="button" class="btn-secondary" onclick={handleRemoveFile}>
          Remove
        </button>
        <button type="button" class="btn-primary" onclick={handleUpload}>
          Upload to IPFS
        </button>
      </div>
    </div>

  {:else if isUploading}
    <!-- Uploading State -->
    <div class="uploading">
      <div class="upload-spinner"></div>
      <div class="upload-text">Uploading to IPFS...</div>
      <div class="upload-progress">
        <div class="progress-bar" style="width: {uploadProgress}%"></div>
      </div>
    </div>

  {:else}
    <!-- Default State: Drop zone -->
    <div
      class="drop-zone"
      class:dragging={isDragging}
      ondragover={handleDragOver}
      ondragleave={handleDragLeave}
      ondrop={handleDrop}
    >
      <div class="drop-icon">📎</div>
      <div class="drop-text">
        <strong>Drag and drop</strong> your evidence file here
      </div>
      <div class="drop-subtext">or</div>
      <label class="btn-select">
        Browse Files
        <input
          type="file"
          accept="image/*,video/*,audio/*,.pdf,.doc,.docx"
          onchange={handleFileSelect}
          disabled={disabled}
        />
      </label>
      <button type="button" class="btn-camera" onclick={handleCameraCapture}>
        📷 Take Photo
      </button>
      <div class="drop-hint">
        Images, videos, PDFs, documents • Max 100MB
      </div>
    </div>

    <!-- Advanced section for manual CID entry -->
    <div class="advanced-section">
      <button
        type="button"
        class="btn-toggle-advanced"
        onclick={() => showAdvanced = !showAdvanced}
      >
        {showAdvanced ? '▼' : '▶'} Advanced: Enter CID manually
      </button>

      {#if showAdvanced}
        <div class="manual-input">
          <input
            type="text"
            bind:value={manualCid}
            placeholder="Qm... or bafy..."
            disabled={disabled}
          />
          <button
            type="button"
            class="btn-primary"
            onclick={handleManualCidSubmit}
            disabled={!manualCid || disabled}
          >
            Use CID
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Error display -->
  {#if error}
    <div class="error-message">
      {error}
    </div>
  {/if}
</div>

<style>
  .file-upload {
    border: 2px dashed #d1d5db;
    border-radius: 8px;
    padding: 1.5rem;
    background: #fafafa;
    transition: all 0.2s;
  }

  .file-upload.disabled {
    opacity: 0.6;
    pointer-events: none;
  }

  /* Drop zone */
  .drop-zone {
    text-align: center;
    padding: 1rem;
  }

  .drop-zone.dragging {
    background: #ede9fe;
    border-color: #667eea;
  }

  .drop-icon {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  .drop-text {
    color: #374151;
    margin-bottom: 0.25rem;
  }

  .drop-subtext {
    color: #9ca3af;
    font-size: 0.875rem;
    margin: 0.5rem 0;
  }

  .btn-select {
    display: inline-block;
    padding: 0.5rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    font-size: 0.875rem;
    margin: 0.5rem 0.5rem 0.5rem 0;
  }

  .btn-select input {
    display: none;
  }

  .btn-camera {
    display: inline-block;
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    font-size: 0.875rem;
    margin: 0.5rem 0;
  }

  .btn-camera:hover {
    background: #e5e7eb;
  }

  .drop-hint {
    color: #9ca3af;
    font-size: 0.75rem;
    margin-top: 0.75rem;
  }

  /* Preview */
  .file-preview {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .preview-image {
    max-width: 200px;
    max-height: 150px;
    border-radius: 8px;
    object-fit: contain;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .preview-icon {
    width: 80px;
    height: 80px;
    background: #e5e7eb;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    color: #6b7280;
  }

  .preview-info {
    text-align: center;
  }

  .preview-name {
    font-weight: 500;
    color: #374151;
    word-break: break-all;
  }

  .preview-size {
    color: #6b7280;
    font-size: 0.875rem;
  }

  .preview-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-secondary {
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
  }

  .btn-primary {
    padding: 0.5rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .btn-primary:hover {
    opacity: 0.9;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Uploading */
  .uploading {
    text-align: center;
    padding: 2rem;
  }

  .upload-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #e5e7eb;
    border-top-color: #667eea;
    border-radius: 50%;
    margin: 0 auto 1rem;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .upload-text {
    color: #374151;
    font-weight: 500;
    margin-bottom: 1rem;
  }

  .upload-progress {
    width: 100%;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    transition: width 0.3s;
  }

  /* Success */
  .upload-success {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    text-align: center;
  }

  .success-icon {
    width: 48px;
    height: 48px;
    background: #d1fae5;
    color: #065f46;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: bold;
  }

  .success-content {
    flex: 1;
  }

  .success-label {
    color: #065f46;
    font-weight: 500;
    margin-bottom: 0.25rem;
  }

  .cid-display {
    font-family: monospace;
    font-size: 0.75rem;
    color: #6b7280;
    background: #f3f4f6;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    word-break: break-all;
    margin-bottom: 0.5rem;
  }

  .view-link {
    color: #667eea;
    font-size: 0.875rem;
    text-decoration: none;
  }

  .view-link:hover {
    text-decoration: underline;
  }

  .btn-clear {
    padding: 0.5rem 1rem;
    background: transparent;
    color: #667eea;
    border: 1px solid #667eea;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .btn-clear:hover {
    background: #ede9fe;
  }

  /* Advanced section */
  .advanced-section {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #e5e7eb;
  }

  .btn-toggle-advanced {
    background: none;
    border: none;
    color: #6b7280;
    font-size: 0.75rem;
    cursor: pointer;
    padding: 0;
  }

  .btn-toggle-advanced:hover {
    color: #374151;
  }

  .manual-input {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.75rem;
  }

  .manual-input input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-family: monospace;
    font-size: 0.875rem;
  }

  /* Error */
  .error-message {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 6px;
    font-size: 0.875rem;
  }

  /* Mobile */
  @media (max-width: 640px) {
    .file-upload {
      padding: 1rem;
    }

    .preview-actions {
      flex-direction: column;
      width: 100%;
    }

    .preview-actions button {
      width: 100%;
    }

    .manual-input {
      flex-direction: column;
    }
  }
</style>
