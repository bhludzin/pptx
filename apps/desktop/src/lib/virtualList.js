// @ts-check

/**
 * Finds the half-open range of positioned rows that intersects the viewport
 * plus an overscan margin. Rows must be ordered by `top`.
 *
 * @param {{ top: number, height: number }[]} rows
 * @param {number} scrollTop
 * @param {number} viewportHeight
 * @param {number} [overscan]
 * @returns {{ start: number, end: number }}
 */
export function visibleRange(rows, scrollTop, viewportHeight, overscan = 400) {
  if (rows.length === 0) return { start: 0, end: 0 }
  const lower = Math.max(0, scrollTop - overscan)
  const upper = Math.max(lower, scrollTop + Math.max(0, viewportHeight) + overscan)

  let low = 0
  let high = rows.length
  while (low < high) {
    const middle = (low + high) >>> 1
    const row = rows[middle]
    if (row.top + row.height <= lower) low = middle + 1
    else high = middle
  }
  const start = low

  high = rows.length
  while (low < high) {
    const middle = (low + high) >>> 1
    if (rows[middle].top < upper) low = middle + 1
    else high = middle
  }
  return { start, end: low }
}
