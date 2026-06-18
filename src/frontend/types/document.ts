export interface DatabaseInfo {
  name: string;
  size_on_disk: number | null;
  empty: boolean | null;
}

export interface DatabaseStats {
  name: string;
  collections: number | null;
  views: number | null;
  objects: number | null;
  avg_obj_size: number | null;
  data_size: number | null;
  storage_size: number | null;
  indexes: number | null;
  index_size: number | null;
}

export interface CollectionInfo {
  name: string;
  type: string | null;
  options: Record<string, unknown> | null;
}

export interface CollectionStats {
  ns: string | null;
  count: number | null;
  size: number | null;
  avg_obj_size: number | null;
  storage_size: number | null;
  total_index_size: number | null;
  total_size: number | null;
  indexes: number | null;
}
