
// Function to format engine names
export function formatEngineName(engineType: string): string {
  switch (engineType) {
    case 'psych':
      return 'Psych Engine';
    case 'kade':
      return 'Kade Engine';
    case 'vanilla':
      return 'Vanilla';
    case 'fpsplus':
      return 'FNF-Plus';
    case 'prevslice':
      return 'Pre-vSlice';
    case 'codename':
      return 'Codename Engine';
    default:
      return engineType; // Return original if unknown
  }
}