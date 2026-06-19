<script lang="ts">
  let {
    open = $bindable(false),
    column = '',
    documents = [],
    currentOp = 'eq',
    currentValue = '',
    onApply,
    onClear,
    hasFilter = false,
  }: {
    open: boolean;
    column: string;
    documents?: any[];
    currentOp?: string;
    currentValue?: string;
    onApply: (op: string, value: string) => void;
    onClear: () => void;
    hasFilter?: boolean;
  } = $props();

  let localOp = $state(currentOp);
  let localValue = $state(currentValue);

  $effect(() => {
    if (open) {
      localOp = currentOp;
      localValue = currentValue;
    }
  });

  function detectFieldType(): string {
    for (const doc of documents) {
      const val = doc[column];
      if (val === null || val === undefined) continue;
      if (typeof val === 'number') return 'number';
      if (typeof val === 'boolean') return 'boolean';
      if (typeof val === 'object' && val.$oid) return 'objectId';
      if (typeof val === 'object' && val.$date) return 'date';
      if (Array.isArray(val)) return 'array';
      if (typeof val === 'object') return 'object';
      return 'string';
    }
    return 'string';
  }

  const fieldType = $derived(detectFieldType());

  interface FilterOp { value: string; label: string; description: string; inputType: string; }

  function getFilterOps(type: string): FilterOp[] {
    switch (type) {
      case 'string':
        return [
          { value: 'eq', label: 'Equals', description: 'Exact match', inputType: 'text' },
          { value: 'ne', label: 'Not equals', description: 'Does not match', inputType: 'text' },
          { value: 'contains', label: 'Contains', description: 'Substring match', inputType: 'text' },
          { value: 'startsWith', label: 'Starts with', description: 'Prefix match', inputType: 'text' },
          { value: 'endsWith', label: 'Ends with', description: 'Suffix match', inputType: 'text' },
          { value: 'regex', label: 'Regex', description: 'Regular expression', inputType: 'text' },
        ];
      case 'number':
        return [
          { value: 'eq', label: 'Equals', description: 'Exact match', inputType: 'number' },
          { value: 'ne', label: 'Not equals', description: 'Does not match', inputType: 'number' },
          { value: 'gt', label: 'Greater than', description: 'Value > threshold', inputType: 'number' },
          { value: 'gte', label: 'Greater or equal', description: 'Value >= threshold', inputType: 'number' },
          { value: 'lt', label: 'Less than', description: 'Value < threshold', inputType: 'number' },
          { value: 'lte', label: 'Less or equal', description: 'Value <= threshold', inputType: 'number' },
          { value: 'between', label: 'Between', description: 'Value between two numbers (comma separated)', inputType: 'number' },
        ];
      case 'date':
        return [
          { value: 'eq', label: 'Equals', description: 'Exact date', inputType: 'date' },
          { value: 'ne', label: 'Not equals', description: 'Different date', inputType: 'date' },
          { value: 'gt', label: 'After', description: 'Date after', inputType: 'date' },
          { value: 'gte', label: 'On or after', description: 'Date on or after', inputType: 'date' },
          { value: 'lt', label: 'Before', description: 'Date before', inputType: 'date' },
          { value: 'lte', label: 'On or before', description: 'Date on or before', inputType: 'date' },
          { value: 'today', label: 'Is today', description: 'Matches today', inputType: 'boolean' },
          { value: 'thisWeek', label: 'Is this week', description: 'Within current week', inputType: 'boolean' },
          { value: 'thisMonth', label: 'Is this month', description: 'Within current month', inputType: 'boolean' },
        ];
      case 'boolean':
        return [
          { value: 'isTrue', label: 'Is true', description: 'Value is true', inputType: 'boolean' },
          { value: 'isFalse', label: 'Is false', description: 'Value is false', inputType: 'boolean' },
          { value: 'exists', label: 'Exists', description: 'Field exists', inputType: 'boolean' },
          { value: 'notExists', label: 'Not exists', description: 'Field does not exist', inputType: 'boolean' },
        ];
      case 'objectId':
        return [
          { value: 'eq', label: 'Equals', description: 'Exact ObjectId match', inputType: 'text' },
        ];
      case 'array':
        return [
          { value: 'contains', label: 'Contains', description: 'Array contains value', inputType: 'text' },
          { value: 'notContains', label: 'Does not contain', description: 'Array missing value', inputType: 'text' },
          { value: 'sizeEquals', label: 'Size equals', description: 'Array length = N', inputType: 'number' },
          { value: 'sizeGt', label: 'Size greater than', description: 'Array length > N', inputType: 'number' },
          { value: 'sizeLt', label: 'Size less than', description: 'Array length < N', inputType: 'number' },
          { value: 'empty', label: 'Is empty', description: 'Array has no elements', inputType: 'boolean' },
          { value: 'notEmpty', label: 'Not empty', description: 'Array has elements', inputType: 'boolean' },
        ];
      case 'object':
        return [
          { value: 'hasKey', label: 'Has key', description: 'Object contains key', inputType: 'text' },
          { value: 'notHasKey', label: 'Does not have key', description: 'Object missing key', inputType: 'text' },
          { value: 'isEmpty', label: 'Is empty', description: 'Object has no keys', inputType: 'boolean' },
          { value: 'notEmpty', label: 'Not empty', description: 'Object has keys', inputType: 'boolean' },
        ];
      default:
        return [
          { value: 'eq', label: 'Equals', description: 'Exact match', inputType: 'text' },
          { value: 'ne', label: 'Not equals', description: 'Does not match', inputType: 'text' },
        ];
    }
  }

  const ops = $derived(getFilterOps(fieldType));
  const currentOpData = $derived(ops.find(o => o.value === localOp) || ops[0]);

  function handleApply() {
    if (['isTrue','isFalse','exists','notExists','today','thisWeek','thisMonth','empty','notEmpty','isEmpty','notHasKey'].includes(localOp)) {
      onApply(localOp, 'true');
    } else {
      onApply(localOp, localValue);
    }
    open = false;
  }

  function handleClear() { onClear(); open = false; }
  function handleKeydown(e: KeyboardEvent) { if (e.key === 'Escape') open = false; if (e.key === 'Enter') handleApply(); }

  const typeColors: Record<string, string> = { string: '#00ED64', number: '#5DD0FF', date: '#B79CFF', boolean: '#FF8966', objectId: '#FFC010', array: '#7E97A7', object: '#7E97A7' };
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16" role="dialog" onkeydown={handleKeydown}>
    <button class="fixed inset-0 bg-black/40" onclick={() => open = false}></button>
    <div class="relative w-full max-w-md rounded-xl border border-[#2D3A45] bg-[#1F2933] shadow-2xl mx-4" onclick={(e) => e.stopPropagation()}>
      <div class="flex items-center justify-between border-b border-[#2D3A45] px-5 py-3">
        <div class="flex items-center gap-2">
          <span class="text-[14px] font-semibold text-[#C3D4DE]">{column}</span>
          <span class="rounded-full px-2 py-0.5 text-[10px] font-medium" style="background-color: {typeColors[fieldType]}20; color: {typeColors[fieldType]}">{fieldType}</span>
        </div>
        <button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => open = false}>
          <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
      <div class="p-5 space-y-4">
        <div>
          <label class="mb-1.5 block text-[11px] font-medium text-[#7E97A7]">Operator</label>
          <select bind:value={localOp} class="w-full rounded-lg border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] cursor-pointer">
            {#each ops as op}<option value={op.value}>{op.label} — {op.description}</option>{/each}
          </select>
        </div>
        {#if !['isTrue','isFalse','exists','notExists','today','thisWeek','thisMonth','empty','notEmpty','isEmpty','notHasKey'].includes(localOp)}
          <div>
            <label class="mb-1.5 block text-[11px] font-medium text-[#7E97A7]">Value</label>
            {#if currentOpData?.inputType === 'number'}
              <input type="number" step="any" bind:value={localValue} placeholder="Enter number..." class="w-full rounded-lg border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64]" />
            {:else if currentOpData?.inputType === 'date'}
              <input type="datetime-local" bind:value={localValue} class="w-full rounded-lg border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64]" />
            {:else}
              <input type="text" bind:value={localValue} placeholder="Enter value..." class="w-full rounded-lg border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64]" />
            {/if}
          </div>
        {/if}
      </div>
      <div class="flex items-center justify-between border-t border-[#2D3A45] px-5 py-3">
        {#if hasFilter}<button class="rounded-lg border border-[#FF5C5C]/30 px-4 py-2 text-[12px] text-[#FF5C5C] hover:bg-[#FF5C5C]/10" onclick={handleClear}>Clear Filter</button>{:else}<div></div>{/if}
        <div class="flex gap-2">
          <button class="rounded-lg border border-[#2D3A45] px-4 py-2 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45]" onclick={() => open = false}>Cancel</button>
          <button class="rounded-lg bg-[#00684A] px-4 py-2 text-[12px] font-medium text-white hover:bg-[#00C75A]" onclick={handleApply}>Apply</button>
        </div>
      </div>
    </div>
  </div>
{/if}
