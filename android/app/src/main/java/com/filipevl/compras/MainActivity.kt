package com.filipevl.compras

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.animation.AnimatedContentTransitionScope
import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.ShoppingCart
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.filipevl.compras.ui.theme.ComprasTheme

@OptIn(ExperimentalMaterial3Api::class)
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            ComprasTheme {
                ComprasApp()
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ComprasApp() {
    ComprasTheme {
        val navController = rememberNavController()
        val navBackStackEntry by navController.currentBackStackEntryAsState()
        val currentRoute = navBackStackEntry?.destination?.route

        val isBottomTabScreen = BottomTab.entries.toTypedArray().any { it.route == currentRoute }

        Scaffold(
            modifier = Modifier.fillMaxSize(),
            topBar = { ComprasTopBar(isVisible = isBottomTabScreen) },
            bottomBar = {
                ComprasBottomBar(
                    currentRoute = currentRoute,
                    isVisible = isBottomTabScreen,
                    onTabSelected = { selectedTab ->
                        navController.navigate(selectedTab.route) {
                            popUpTo(navController.graph.findStartDestination().id) {
                                saveState = true
                            }
                            launchSingleTop = true
                            restoreState = true
                        }
                    }
                )
            }
        ) { innerPadding ->
            NavHost(
                navController = navController,
                startDestination = BottomTab.Home.route,
                modifier = Modifier.padding(innerPadding),
                enterTransition = {
                    val initialIndex = getTabIndex(initialState.destination.route)
                    val targetIndex = getTabIndex(targetState.destination.route)

                    if (initialIndex != -1 && targetIndex != -1) {
                        if (targetIndex > initialIndex) {
                            slideIntoContainer(AnimatedContentTransitionScope.SlideDirection.Left, tween(300))
                        } else {
                            slideIntoContainer(AnimatedContentTransitionScope.SlideDirection.Right, tween(300))
                        }
                    } else {
                        slideIntoContainer(AnimatedContentTransitionScope.SlideDirection.Up, tween(300))
                    }
                },
                exitTransition = {
                    val initialIndex = getTabIndex(initialState.destination.route)
                    val targetIndex = getTabIndex(targetState.destination.route)

                    if (initialIndex != -1 && targetIndex != -1) {
                        if (targetIndex > initialIndex) {
                            slideOutOfContainer(AnimatedContentTransitionScope.SlideDirection.Left, tween(300))
                        } else {
                            slideOutOfContainer(AnimatedContentTransitionScope.SlideDirection.Right, tween(300))
                        }
                    } else {
                        slideOutOfContainer(AnimatedContentTransitionScope.SlideDirection.Down, tween(300))
                    }
                }
            ) {
                // --- TAB SCREENS ---
                composable(BottomTab.Home.route) {
                    HomeScreen(onNavigateToDetails = { navController.navigate("details") })
                }
                composable(BottomTab.Search.route) { SearchScreen() }
                composable(BottomTab.Profile.route) { ProfileScreen() }

                // --- FULL SCREEN SCREENS ---
                composable("details") {
                    DetailsScreen(onBack = { navController.popBackStack() })
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ComprasTopBar(isVisible: Boolean) {
    if (!isVisible) return

    TopAppBar(
        title = {
            Text(
                text = "Compras",
                style = MaterialTheme.typography.titleLarge,
                color = if (isSystemInDarkTheme()) Color.White else Color.Black,
            )
        },
        navigationIcon = {
            Icon(
                imageVector = Icons.Default.ShoppingCart,
                contentDescription = "Compras App Logo",
                modifier = Modifier.padding(start = 16.dp, end = 8.dp),
                tint = MaterialTheme.colorScheme.primary
            )
        }
    )
}


@Composable
fun ComprasBottomBar(
    currentRoute: String?,
    isVisible: Boolean,
    onTabSelected: (BottomTab) -> Unit
) {
    if (!isVisible) return

    NavigationBar {
        BottomTab.entries.forEach { tab ->
            NavigationBarItem(
                icon = { Icon(tab.icon, contentDescription = tab.title) },
                label = { Text(tab.title) },
                selected = currentRoute == tab.route,
                // Fire the callback, let the router handle the actual navigation
                onClick = { onTabSelected(tab) }
            )
        }
    }
}

enum class BottomTab(val route: String, val title: String, val index: Int, val icon: ImageVector) {
    Home("home", "Home", 0, Icons.Default.Home),
    Search("search", "Search", 1, Icons.Default.Search),
    Profile("profile", "Profile", 2, Icons.Default.Person)
}

fun getTabIndex(route: String?): Int {
    return BottomTab.values().find { it.route == route }?.index ?: -1
}

@Composable
fun HomeScreen(onNavigateToDetails: () -> Unit) {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text("Home Screen", style = MaterialTheme.typography.headlineMedium)
            Spacer(modifier = Modifier.height(16.dp))
            Button(onClick = onNavigateToDetails) {
                Text("View Product Details")
            }
        }
    }
}

@Composable
fun SearchScreen() {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Text("Search Screen", style = MaterialTheme.typography.headlineMedium)
    }
}

@Composable
fun ProfileScreen() {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Text("Profile Screen", style = MaterialTheme.typography.headlineMedium)
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DetailsScreen(onBack: () -> Unit) {
    // Since this is a full-screen view, we provide a localized Scaffold
    // with a dedicated back button to get back to the Compras dashboard.
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Product Details") },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { padding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding),
            contentAlignment = Alignment.Center
        ) {
            Text("The main Compras TopBar and BottomBar are hidden!", style = MaterialTheme.typography.bodyLarge)
        }
    }
}