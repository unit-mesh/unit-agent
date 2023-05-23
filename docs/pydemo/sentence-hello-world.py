from sentence_transformers import SentenceTransformer

sentences = ["Hello, world!"]

model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')
embeddings = model.encode(sentences)
print(embeddings)
