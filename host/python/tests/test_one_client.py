import unittest
import os.path

# host.python.src.one_sdk.client
from one_sdk import OneClient, UnexpectedError

from echo_server import EchoHttpServer

ASSETS_PATH = os.path.abspath(os.path.join(__file__, "../../../../examples/comlinks/src"))

class TestOneClient(unittest.TestCase):
	@classmethod
	def setUpClass(cls):
		cls._http_server = EchoHttpServer()
		cls._http_server.run_threaded()
	
	@classmethod
	def tearDownClass(cls):
		cls._http_server.shutdown()

	def test_basic_use(self):
		client = OneClient(assets_path = ASSETS_PATH, superface_api_url = "superface.localhost")

		profile = client.get_profile("wasm-sdk/example")
		result = profile.get_usecase("Example").perform(
			{ "id": 1 },
			provider = "localhost",
			parameters = { "PARAM": "parameter_value" },
			security = { "basic_auth": { "username": "username", "password": "password" } }
		)

		self.assertTrue("/api/1" in result["url"])

	def test_destroy_without_setup(self):
		client = OneClient(assets_path = ASSETS_PATH, superface_api_url = "superface.localhost")
		client.destroy()
		self.assertTrue(True)

	def test_panicked_core(self):
		client = OneClient(assets_path = ASSETS_PATH, superface_api_url = "superface.localhost")
		client._internal._core_path = os.path.abspath(os.path.join(__file__, "../../assets/test-core.wasm"))

		profile = client.get_profile("wasm-sdk/example")
		self.assertRaises(
			UnexpectedError,
			lambda: profile.get_usecase("CORE_PERFORM_PANIC").perform({}, provider = "localhost")
		)
		self.assertTrue(
			profile.get_usecase("CORE_PERFORM_TRUE").perform({}, provider = "localhost")
		)

if __name__ == '__main__':
	unittest.main()