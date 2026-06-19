import type { DocumentPage } from '../types/document';
import { tauriInvoke } from '../services/tauriBridge';

class DocumentStore {
  documents: any[] = [];
  totalCount = 0;
  page = 0;
  pageSize = 50;
  loading = false;
  connectionId = '';
  database = '';
  collection = '';

  async loadDocuments(connectionId: string, database: string, collection: string, page: number = 0, filter?: string, sort?: string) {
    this.connectionId = connectionId;
    this.database = database;
    this.collection = collection;
    this.page = page;
    this.loading = true;

    const { data, error } = await tauriInvoke<DocumentPage>('find_documents', {
      params: {
        connection_id: connectionId,
        database,
        collection,
        filter: filter || null,
        sort: sort || null,
        page,
        page_size: this.pageSize,
      }
    });

    if (!error && data) {
      this.documents = data.documents;
      this.totalCount = data.total_count;
    } else {
      this.documents = [];
      this.totalCount = 0;
    }
    this.loading = false;
  }

  async insertDocument(doc: string) {
    const { error } = await tauriInvoke('insert_document', {
      params: {
        connection_id: this.connectionId,
        database: this.database,
        collection: this.collection,
        document: doc,
      }
    });
    return !error;
  }

  async updateDocument(id: string, update: string) {
    const { error } = await tauriInvoke('update_document', {
      params: {
        connection_id: this.connectionId,
        database: this.database,
        collection: this.collection,
        id,
        update,
      }
    });
    return !error;
  }

  async deleteDocument(id: string) {
    const { error } = await tauriInvoke('delete_document', {
      params: {
        connection_id: this.connectionId,
        database: this.database,
        collection: this.collection,
        id,
      }
    });
    return !error;
  }

  reset() {
    this.documents = [];
    this.totalCount = 0;
    this.page = 0;
    this.connectionId = '';
    this.database = '';
    this.collection = '';
  }
}

export const documentStore = new DocumentStore();
