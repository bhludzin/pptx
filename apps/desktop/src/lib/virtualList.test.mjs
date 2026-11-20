import assert from 'node:assert/strict'
import test from 'node:test'

import { visibleRange } from './virtualList.js'

const rows = Array.from({ length: 35_000 }, (_, index) => ({
  top: index * 100,
  height: 100,
}))

test('large lists return only viewport-adjacent rows', () => {
  const range = visibleRange(rows, 1_700_000, 600, 200)
  assert.deepEqual(range, { start: 16_998, end: 17_008 })
  assert.ok(range.end - range.start < 20)
})

test('range clamps at the beginning and end', () => {
  assert.deepEqual(visibleRange(rows, 0, 250, 0), { start: 0, end: 3 })
  assert.deepEqual(visibleRange(rows, 3_499_900, 500, 0), { start: 34_999, end: 35_000 })
})

test('empty lists return an empty range', () => {
  assert.deepEqual(visibleRange([], 100, 100), { start: 0, end: 0 })
})
