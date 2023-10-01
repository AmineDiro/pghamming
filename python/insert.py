import asyncio
import secrets
from time import perf_counter
from uuid import uuid4

import asyncpg
import numpy as np
from pgvector.asyncpg import register_vector

N_BYTES = 8
N_ROW = 1_000_000


def byte_to_vec(byte):
    # Create a NumPy array directly from the random bytes
    return (
        np.unpackbits(np.frombuffer(byte, dtype=np.uint8))
        .astype(np.int32)
        .flatten()
    )


async def insert_vec(conn, data):
    await conn.executemany(
        """
        INSERT INTO images(id,phash) VALUES($1, $2)
    """,
        [(uuid, byte_to_vec(phash)) for uuid, phash in data],
    )


async def insert_bin(conn, data):
    await conn.executemany(
        """
        INSERT INTO images_bin(id, phash) VALUES($1, $2)
    """,
        data,
    )


async def main():
    conn = await asyncpg.connect(
        "postgresql://postgres:postgres@localhost:5500/postgres"
    )
    await conn.execute("CREATE EXTENSION IF NOT EXISTS vector")
    await register_vector(conn)

    await conn.execute(
        """
        DROP TABLE IF EXISTS  images;
        CREATE TABLE images(
            id  UUID PRIMARY KEY,
            phash vector(64)
        )
    """
    )
    await conn.execute(
        """
        DROP TABLE IF EXISTS  images_bin;
        CREATE TABLE images_bin(
            id  UUID PRIMARY KEY,
            phash bytea
        )
    """
    )

    data = [(uuid4(), secrets.token_bytes(N_BYTES)) for _ in range(N_ROW)]
    await insert_vec(conn, data)
    await insert_bin(conn, data)

    # Close the connection.
    await conn.close()


if __name__ == "__main__":
    s = perf_counter()
    asyncio.run(main())
    e = perf_counter()

    print(f"Inserting {N_ROW} took {e-s:.2f}s")
