export class PaxService {
  private static readonly SPECIAL_MAP: Record<string, string> = {
    'Fun Class': 'FUN',
  };

  private paxMultipliers: Record<string, number> = { FUN: 4 };

  async init(): Promise<void> {
    const html = document.createElement('html');
    html.innerHTML = await PaxService.getPaxHtmlDoc();
    const rows = Array.from(
      html
        .querySelector<HTMLTableElement>('body table')!
        .querySelectorAll<HTMLTableRowElement>('tr'),
    );
    rows
      .map((row) =>
        Array.from(row.querySelectorAll<HTMLTableDataCellElement>('td')),
      )
      .forEach((row: HTMLTableDataCellElement[]) => {
        const stringRow = row
          .map((r) => r.textContent || '')
          .map((s) => s.trim());
        for (let i = 0; i < stringRow.length; i++) {
          const classAbbr = stringRow[i];
          if (classAbbr) {
            try {
              this.paxMultipliers[classAbbr.replaceAll('-', '')] = parseFloat(
                row[++i].textContent!,
              );
            } catch (e) {
              console.error(
                `${e} for class=${classAbbr} i=${i} ${JSON.stringify(
                  stringRow,
                )}`,
              );
            }
          }
        }
      });

    console.log(JSON.stringify(this.paxMultipliers, null, 2));
  }

  getMultiplierFromLongName(longClassName: string): number {
    if (longClassName.toLocaleLowerCase().endsWith('ladies'))
      longClassName.substring(0, longClassName.length - 'ladies'.length).trim();
    try {
      return this.getMultiplierFromAbbrName(
        longClassName
          .split(' ')
          .map((word) => word[0])
          .join('')
          .toUpperCase(),
      );
    } catch (_) {
      const abbreviatedName = PaxService.SPECIAL_MAP[longClassName];
      if (abbreviatedName)
        return this.getMultiplierFromAbbrName(abbreviatedName);
      else throw new Error(`No multiplier found for ${longClassName}`);
    }
  }

  getMultiplierFromAbbrName(abbrClassName: string): number {
    if (abbrClassName.toUpperCase().endsWith('L'))
      abbrClassName = abbrClassName.substring(0, abbrClassName.length - 1);

    const result = this.paxMultipliers[abbrClassName];
    if (result) return result;
    else throw new Error(`No multiplier found for ${abbrClassName}`);
  }

  private static async getPaxHtmlDoc(): Promise<string> {
    const results = await fetch('pax-index.html');
    if (results.ok) {
      return results.text();
    } else {
      throw new Error(
        `Failed to fetch PAX multipliers: ${results.status} / ${results.statusText}`,
      );
    }
  }
}
