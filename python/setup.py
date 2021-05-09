from setuptools import setup

setup(
    version='0.1.0',
    name='palantir-proto-py',
    description='Compiled protocol buffer for palantir project',
    author='Oleksandr Korienev',
    author_email='alexkorienev@gmail.com',
    url='https://github.com/AlexPraefectus/palantir-proto',
    package_dir={'': 'generated'},
    install_requires=[
        'protobuf==3.16.0'
    ],
)
