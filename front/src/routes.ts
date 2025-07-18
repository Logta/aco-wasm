import { createRootRoute, createRoute } from "@tanstack/react-router";
import App from "./App";
import EducationPage from "./pages/EducationPage";
import TSPPage from "./pages/TSPPage";

export const rootRoute = createRootRoute({
  component: App,
});

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: TSPPage,
});

export const tspRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/tsp",
  component: TSPPage,
});

export const educationRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/education",
  component: EducationPage,
});

export const routeTree = rootRoute.addChildren([indexRoute, tspRoute, educationRoute]);
