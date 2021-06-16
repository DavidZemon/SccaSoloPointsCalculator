export const SPECIAL_CLASS_MAP: Record<string, string> = {
  'Fun Class': 'FUN',
};

export function toShortClassName(longClassName: string): string {
  return (
    SPECIAL_CLASS_MAP[longClassName] ||
    longClassName
      .split(' ')
      .map((word) => word[0])
      .join('')
  );
}
