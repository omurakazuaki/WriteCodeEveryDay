import { NativeModules as RNNativeModules } from "react-native";
RNNativeModules.UIManager = RNNativeModules.UIManager || {};
RNNativeModules.UIManager.RCTView = RNNativeModules.UIManager.RCTView || {};
RNNativeModules.RNGestureHandlerModule = RNNativeModules.RNGestureHandlerModule || {
  State: { BEGAN: "BEGAN", FAILED: "FAILED", ACTIVE: "ACTIVE", END: "END" },
  attachGestureHandler: jest.fn(),
  createGestureHandler: jest.fn(),
  dropGestureHandler: jest.fn(),
  updateGestureHandler: jest.fn(),

};
RNNativeModules.PlatformConstants = RNNativeModules.PlatformConstants || {
  forceTouchAvailable: false
};
jest.mock('@react-native-community/async-storage', () => jest.genMockFromModule('@react-native-community/async-storage'));
